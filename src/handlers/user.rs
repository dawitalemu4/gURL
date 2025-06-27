use axum::{
    Json,
    extract::{self, State},
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
};
use miette::{Result, miette};

use crate::handlers::{ConnectionState, RequestBody, map_user, serialize_favorites_for_db};
use crate::utils::{
    extract_token_from_header, generate_jwt, hash_password, validate_jwt, verify_password,
};

pub async fn get_user(
    State(state): ConnectionState,
    extract::Json(body): extract::Json<RequestBody>,
) -> impl IntoResponse {
    let result: Result<(StatusCode, Json<Vec<crate::models::user::User>>)> = (|| {
        let user = body.user.clone().unwrap();
        let db = state
            .lock()
            .map_err(|e| miette!("Global db can't block current thread {e}"))?;

        let user_rows = db
            .prepare("SELECT * FROM user WHERE email = ?1 AND deleted = false")
            .map_err(|e| miette!("Database error: {e}"))?;
        let db_users = map_user(user_rows, &[user.email.clone()])?;

        if db_users.is_empty() {
            return Ok((StatusCode::NOT_FOUND, Json(vec![])));
        }

        let db_user = &db_users[0];

        let password_valid = verify_password(&user.password, &db_user.password)
            .map_err(|e| miette!("Password verification failed: {e}"))?;

        if !password_valid {
            return Ok((StatusCode::UNAUTHORIZED, Json(vec![])));
        }

        let token =
            generate_jwt(&user.email).map_err(|e| miette!("Failed to generate JWT: {e}"))?;

        let mut response_user = db_user.clone();
        response_user.password = token; // Use password field to return JWT token

        Ok((StatusCode::OK, Json(vec![response_user])))
    })();

    match result {
        Ok(response) => response.into_response(),
        Err(e) => {
            eprintln!("Error in get_user: {e:?}");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json("Internal server error"),
            )
                .into_response()
        }
    }
}

pub async fn create_user(
    State(state): ConnectionState,
    extract::Json(body): extract::Json<RequestBody>,
) -> impl IntoResponse {
    let result: Result<(StatusCode, Json<Vec<crate::models::user::User>>)> = (|| {
        let user = body.user.clone().unwrap();
        let db = state
            .lock()
            .map_err(|e| miette!("Global db can't block current thread {e}"))?;

        let hashed_password =
            hash_password(&user.password).map_err(|e| miette!("Failed to hash password: {e}"))?;

        match db.prepare("INSERT INTO user (username, email, password, favorites, date, oldPassword, deleted) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7) RETURNING *") {
            Ok(rows) => {
                let res = map_user(
                    rows,
                    &[user.username, user.email, hashed_password, serialize_favorites_for_db(&user.favorites), user.date.unwrap_or_default(), user.old_pw, user.deleted.to_string()],
                )?;
                if res.is_empty() {
                    Ok((StatusCode::NOT_FOUND, Json(vec![])))
                } else {
                    Ok((StatusCode::OK, Json(res)))
                }
            }
            Err(e) => Ok((StatusCode::INTERNAL_SERVER_ERROR, Json(vec![]))),
        }
    })();

    match result {
        Ok(response) => response.into_response(),
        Err(e) => {
            eprintln!("Error in create_user: {e:?}");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json("Internal server error"),
            )
                .into_response()
        }
    }
}

pub async fn update_user(
    State(state): ConnectionState,
    headers: HeaderMap,
    extract::Json(body): extract::Json<RequestBody>,
) -> impl IntoResponse {
    let result: Result<(StatusCode, Json<Vec<crate::models::user::User>>)> = (|| {
        let user = body.user.clone().unwrap();
        let db = state
            .lock()
            .map_err(|e| miette!("Global db can't block current thread {e}"))?;

        // Validate JWT token
        let auth_header = headers
            .get("authorization")
            .ok_or_else(|| miette!("Authorization header missing"))?
            .to_str()
            .map_err(|e| miette!("Invalid authorization header: {e}"))?;

        let token = extract_token_from_header(auth_header)?;
        let claims = validate_jwt(&token)?;

        // Ensure user can only update their own account
        if claims.sub != user.email {
            return Ok((StatusCode::FORBIDDEN, Json(vec![])));
        }

        // Hash the new password if provided
        let password_to_store = if user.password != user.old_pw {
            hash_password(&user.password).map_err(|e| miette!("Failed to hash password: {e}"))?
        } else {
            user.password.clone()
        };

        match db.prepare("UPDATE user SET username = ?1, password = ?2, oldPassword = ?3 WHERE email = ?4 AND deleted = false RETURNING *") {
            Ok(rows) => {
                let res = map_user(rows, &[user.username, password_to_store, user.old_pw, user.email])?;
                if res.is_empty() {
                    Ok((StatusCode::NOT_FOUND, Json(vec![])))
                } else {
                    Ok((StatusCode::OK, Json(res)))
                }
            }
            Err(e) => Ok((StatusCode::INTERNAL_SERVER_ERROR, Json(vec![]))),
        }
    })();

    match result {
        Ok(response) => response.into_response(),
        Err(e) => {
            eprintln!("Error in update_user: {e:?}");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json("Internal server error"),
            )
                .into_response()
        }
    }
}

pub async fn delete_user(
    State(state): ConnectionState,
    headers: HeaderMap,
    extract::Json(body): extract::Json<RequestBody>,
) -> impl IntoResponse {
    let result: Result<(StatusCode, Json<Vec<crate::models::user::User>>)> = (|| {
        let user = body.user.clone().unwrap();
        let db = state
            .lock()
            .map_err(|e| miette!("Global db can't block current thread {e}"))?;

        // Validate JWT token
        let auth_header = headers
            .get("authorization")
            .ok_or_else(|| miette!("Authorization header missing"))?
            .to_str()
            .map_err(|e| miette!("Invalid authorization header: {e}"))?;

        let token = extract_token_from_header(auth_header)?;
        let claims = validate_jwt(&token)?;

        // Ensure user can only delete their own account
        if claims.sub != user.email {
            return Ok((StatusCode::FORBIDDEN, Json(vec![])));
        }

        match db.prepare("UPDATE user SET deleted = true WHERE email = ?1 RETURNING *") {
            Ok(rows) => {
                let res = map_user(rows, &[user.email])?;
                if res.is_empty() {
                    Ok((StatusCode::NOT_FOUND, Json(vec![])))
                } else {
                    Ok((StatusCode::OK, Json(res)))
                }
            }
            Err(e) => Ok((StatusCode::INTERNAL_SERVER_ERROR, Json(vec![]))),
        }
    })();

    match result {
        Ok(response) => response.into_response(),
        Err(e) => {
            eprintln!("Error in delete_user: {e:?}");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json("Internal server error"),
            )
                .into_response()
        }
    }
}

pub async fn update_favorites(
    State(state): ConnectionState,
    headers: HeaderMap,
    extract::Json(body): extract::Json<RequestBody>,
) -> impl IntoResponse {
    let result: Result<(StatusCode, Json<Vec<crate::models::user::User>>)> = (|| {
        let user = body.user.clone().unwrap();
        let db = state
            .lock()
            .map_err(|e| miette!("Global db can't block current thread {e}"))?;

        // Validate JWT token
        let auth_header = headers
            .get("authorization")
            .ok_or_else(|| miette!("Authorization header missing"))?
            .to_str()
            .map_err(|e| miette!("Invalid authorization header: {e}"))?;

        let token = extract_token_from_header(auth_header)?;
        let claims = validate_jwt(&token)?;

        // Ensure user can only update their own favorites
        if claims.sub != user.email {
            return Ok((StatusCode::FORBIDDEN, Json(vec![])));
        }

        match db.prepare(
            "UPDATE user SET favorites = ?1 WHERE email = ?2 AND deleted = false RETURNING *",
        ) {
            Ok(rows) => {
                let res = map_user(
                    rows,
                    &[serialize_favorites_for_db(&user.favorites), user.email],
                )?;
                if res.is_empty() {
                    Ok((StatusCode::NOT_FOUND, Json(vec![])))
                } else {
                    Ok((StatusCode::OK, Json(res)))
                }
            }
            Err(e) => Ok((StatusCode::INTERNAL_SERVER_ERROR, Json(vec![]))),
        }
    })();

    match result {
        Ok(response) => response.into_response(),
        Err(e) => {
            eprintln!("Error in update_favorites: {e:?}");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json("Internal server error"),
            )
                .into_response()
        }
    }
}
