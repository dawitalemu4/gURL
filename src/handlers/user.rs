use axum::{
    extract::{Json, State},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use miette::{Result, miette};

use crate::utils::{create_jwt, hash_password, verify_password};
use crate::{
    handlers::{ConnectionState, map_user},
    models::{serialize_bool_for_db, serialize_favorites_for_db, user::User},
};

pub async fn get_user(State(state): ConnectionState, Json(user): Json<User>) -> Response {
    let res: Result<Response> = (|| {
        let (email, password) = (user.email, user.password);
        let db = state
            .lock()
            .map_err(|e| miette!("Global db can't block current thread {e}"))?;

        match map_user(
            db.prepare(r#"SELECT * FROM "user" WHERE email = ?1 AND deleted = false"#)
                .map_err(|e| miette!("Invalid statement: {e}"))?,
            &[&email],
        ) {
            Ok(mapped_user) => {
                if let Some(parsed_user) = mapped_user.first() {
                    if parsed_user.deleted {
                        return Ok((StatusCode::NOT_FOUND).into_response());
                    }

                    match verify_password(&password, &parsed_user.password) {
                        Ok(password_valid) => {
                            if !password_valid {
                                return Ok((StatusCode::UNAUTHORIZED).into_response());
                            }

                            Ok((StatusCode::OK, Json(create_jwt(parsed_user.to_owned())?))
                                .into_response())
                        }
                        Err(e) => Ok((
                            StatusCode::INTERNAL_SERVER_ERROR,
                            format!("Server Error: {e}"),
                        )
                            .into_response()),
                    }
                } else {
                    Ok((StatusCode::NOT_FOUND,).into_response())
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

pub async fn create_user(State(state): ConnectionState, Json(user): Json<User>) -> Response {
    let res: Result<Response> = (|| {
        let db = state
            .lock()
            .map_err(|e| miette!("Global db can't block current thread {e}"))?;

        let hashed_password = hash_password(&user.password)?;

        match map_user(
            db.prepare(
                r#"
                INSERT INTO "user" (username, email, password, favorites, date, deleted) 
                VALUES (?1, ?2, ?3, ?4, ?5, ?6) RETURNING *
            "#,
            )
            .map_err(|e| miette!("Invalid statement: {e}"))?,
            &[
                &user.username,
                &user.email,
                &hashed_password,
                &serialize_favorites_for_db(&user.favorites),
                &user.date.clone().unwrap_or_default(),
                &serialize_bool_for_db(user.deleted).to_string(),
            ],
        ) {
            Ok(_) => Ok((StatusCode::OK, Json(create_jwt(user)?)).into_response()),
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

pub async fn update_user(State(state): ConnectionState, Json(mut user): Json<User>) -> Response {
    let res: Result<Response> = (|| {
        let db = state
            .lock()
            .map_err(|e| miette!("Global db can't block current thread {e}"))?;

        user.password = hash_password(&user.password)?;

        match map_user(db
            .prepare(r#"UPDATE "user" SET username = ?1, password = ?2 WHERE email = ?3"#)
            .map_err(|e| miette!("Invalid statement: {e}"))?, &[&user.username, &user.password, &user.email])
        {
            Ok(_) => {
                Ok((StatusCode::OK, Json(create_jwt(user)?)).into_response())
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

pub async fn delete_user(State(state): ConnectionState, Json(user): Json<User>) -> Response {
    let res: Result<Response> = (|| {
        let db = state
            .lock()
            .map_err(|e| miette!("Global db can't block current thread {e}"))?;

        match map_user(db
            .prepare(r#"UPDATE "user" SET deleted = true WHERE email = ?1 RETURNING *"#)
            .map_err(|e| miette!("Invalid statement: {e}"))?, &[&user.email])
        {
            Ok(_) => {
                Ok((StatusCode::OK, Json(true)).into_response())
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

pub async fn update_favorites(State(state): ConnectionState, Json(user): Json<User>) -> Response {
    let res: Result<Response> = (|| {
        let db = state
            .lock()
            .map_err(|e| miette!("Global db can't block current thread {e}"))?;

        match map_user(db
            .prepare(r#"UPDATE "user" SET favorites = ?1 WHERE email = ?2 AND deleted = false"#)
            .map_err(|e| miette!("Invalid statement: {e}"))?,
                    &[&serialize_favorites_for_db(&user.favorites), &user.email],
                )
        {
            Ok(mapped_user) => {
                if let Some(parsed_user) = mapped_user.first() {
                    Ok((StatusCode::OK, Json(create_jwt(parsed_user.clone())?)).into_response())
                } else {
                    Ok((StatusCode::BAD_REQUEST).into_response())

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
