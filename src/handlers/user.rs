use axum::{
    extract::{Json, State},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use miette::{Result, miette};

use crate::handlers::{ConnectionState, RequestBody, map_user, serialize_favorites_for_db};
use crate::utils::{create_jwt, hash_password, verify_password};

pub async fn get_user(State(state): ConnectionState, Json(body): Json<RequestBody>) -> Response {
    let res: Result<Response> = (|| {
        let user = body.user.expect("Cannot serialize User from body");
        let (email, password) = (user.email, user.password);
        let db = state
            .lock()
            .map_err(|e| miette!("Global db can't block current thread {e}"))?;

        match db.prepare(r#"SELECT * FROM "user" WHERE email = ?1 AND deleted = false"#) {
            Ok(rows) => {
                let parsed_user = map_user(rows, &[email])?;

                if parsed_user.deleted {
                    return Ok((StatusCode::NOT_FOUND).into_response());
                }

                match verify_password(&password, &parsed_user.password) {
                    Ok(password_valid) => {
                        if !password_valid {
                            return Ok((StatusCode::UNAUTHORIZED).into_response());
                        }

                        Ok((StatusCode::OK, create_jwt(parsed_user)?).into_response())
                    }
                    Err(e) => Ok((
                        StatusCode::INTERNAL_SERVER_ERROR,
                        format!("Server Error: {e}"),
                    )
                        .into_response()),
                }
            }
            Err(e) => Ok((
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Server Error: {e}"),
            )
                .into_response()),
        }
    })();

    match res {
        Ok(res) => res,
        Err(e) => panic!("{e}"),
    }
}

pub async fn create_user(State(state): ConnectionState, Json(body): Json<RequestBody>) -> Response {
    let res: Result<Response> = (|| {
        let user = body.user.expect("Cannot serialize User from body");
        let db = state
            .lock()
            .map_err(|e| miette!("Global db can't block current thread {e}"))?;

        let hashed_password = hash_password(&user.password)?;

        match db.prepare(
            r#"
                INSERT INTO "user" (username, email, password, favorites, date, oldPassword, deleted) 
                VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7) RETURNING *
            "#,
        ) {
            Ok(rows) => {
                let user = map_user(
                    rows,
                    &[
                        user.username,
                        user.email,
                        hashed_password,
                        serialize_favorites_for_db(&user.favorites),
                        user.date.unwrap_or_default(),
                        user.old_pw,
                        user.deleted.to_string(),
                    ],
                )?;

                Ok((StatusCode::OK, create_jwt(user)?).into_response())
            }
            Err(e) => Ok((
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Server Error: {e}"),
            )
                .into_response()),
        }
    })();

    match res {
        Ok(res) => res,
        Err(e) => panic!("{e}"),
    }
}

pub async fn update_user(State(state): ConnectionState, Json(body): Json<RequestBody>) -> Response {
    let res: Result<Response> = (|| {
        let mut user = body.user.expect("Cannot serialize User from body");
        let db = state
            .lock()
            .map_err(|e| miette!("Global db can't block current thread {e}"))?;

        user.password = hash_password(&user.password)?;

        match db.prepare(r#"UPDATE "user" SET username = ?1, password = ?2 WHERE email = ?3"#) {
            Ok(rows) => {
                let user = map_user(rows, &[user.username, user.password, user.email])?;

                Ok((StatusCode::OK, create_jwt(user)?).into_response())
            }
            Err(e) => Ok((
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Server Error: {e}"),
            )
                .into_response()),
        }
    })();

    match res {
        Ok(res) => res,
        Err(e) => panic!("{e}"),
    }
}

pub async fn delete_user(State(state): ConnectionState, Json(body): Json<RequestBody>) -> Response {
    let res: Result<Response> = (|| {
        let user = body.user.expect("Cannot serialize User from body");
        let db = state
            .lock()
            .map_err(|e| miette!("Global db can't block current thread {e}"))?;

        match db.prepare(r#"UPDATE "user" SET deleted = true WHERE email = ?1 RETURNING *"#) {
            Ok(rows) => {
                map_user(rows, &[user.email])?;

                Ok((StatusCode::OK).into_response())
            }
            Err(e) => Ok((
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Server Error: {e}"),
            )
                .into_response()),
        }
    })();

    match res {
        Ok(res) => res,
        Err(e) => panic!("{e}"),
    }
}

pub async fn update_favorites(
    State(state): ConnectionState,
    Json(body): Json<RequestBody>,
) -> Response {
    let res: Result<Response> = (|| {
        let user = body.user.expect("Cannot serialize User from body");
        let db = state
            .lock()
            .map_err(|e| miette!("Global db can't block current thread {e}"))?;

        match db.prepare(r#"UPDATE "user" SET favorites = ?1 WHERE email = ?2 AND deleted = false"#)
        {
            Ok(rows) => {
                let user = map_user(
                    rows,
                    &[serialize_favorites_for_db(&user.favorites), user.email],
                )?;

                Ok((StatusCode::OK, create_jwt(user)?).into_response())
            }
            Err(e) => Ok((
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Server Error: {e}"),
            )
                .into_response()),
        }
    })();

    match res {
        Ok(res) => res,
        Err(e) => panic!("{e}"),
    }
}
