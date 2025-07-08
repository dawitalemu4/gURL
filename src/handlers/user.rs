use axum::{
    extract::{Json, State},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use miette::{Result, miette};

use crate::utils::{create_jwt, hash_password, verify_password};
use crate::{
    handlers::{ConnectionState, map_user},
    models::{user::User, serialize_favorites_for_db, serialize_bool_for_db}
};

pub async fn get_user(State(state): ConnectionState, Json(user): Json<User>) -> Response {
    let res: Result<Response> = (|| {
        let (email, password) = (user.email, user.password);
        let db = state
            .lock()
            .map_err(|e| miette!("Global db can't block current thread {e}"))?;

        match db.prepare(r#"SELECT * FROM "user" WHERE email = ?1 AND deleted = false"#) {
            Ok(rows) => match map_user(rows, &[email]) {
                Ok(parsed_user) => {
                    if parsed_user.deleted {
                        return Ok((StatusCode::NOT_FOUND).into_response());
                    }

                    match verify_password(&password, &parsed_user.password) {
                        Ok(password_valid) => {
                            if !password_valid {
                                return Ok((StatusCode::UNAUTHORIZED).into_response());
                            }

                            Ok((StatusCode::OK, Json(create_jwt(parsed_user)?)).into_response())
                        }
                        Err(e) => Ok((
                            StatusCode::INTERNAL_SERVER_ERROR,
                            format!("Server Error: {e}"),
                        )
                            .into_response()),
                    }
                }
                Err(e) => Ok((
                    StatusCode::NOT_FOUND,
                    Json(format!("{e}: user doesn't exist or incorrect credentials")),
                )
                    .into_response()),
            },
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
    let res: Result<Response> =
        (|| {
            let db = state
                .lock()
                .map_err(|e| miette!("Global db can't block current thread {e}"))?;

            let hashed_password = hash_password(&user.password)?;

            match db.prepare(
                r#"
                INSERT INTO "user" (username, email, password, favorites, date, deleted) 
                VALUES (?1, ?2, ?3, ?4, ?5, ?6) RETURNING *
            "#,
            ) {
                Ok(rows) => {
                    match map_user(
                        rows,
                        &[
                            user.username,
                            user.email,
                            hashed_password,
                            serialize_favorites_for_db(&user.favorites),
                            user.date.unwrap_or_default(),
                            serialize_bool_for_db(user.deleted).to_string(),
                        ],
                    ) {
                        Ok(user) => Ok((StatusCode::OK, Json(create_jwt(user)?)).into_response()),
                        Err(e) => Ok((StatusCode::NOT_FOUND, Json(format!("{e}")))
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

pub async fn update_user(State(state): ConnectionState, Json(mut user): Json<User>) -> Response {
    let res: Result<Response> = (|| {
        let db = state
            .lock()
            .map_err(|e| miette!("Global db can't block current thread {e}"))?;

        user.password = hash_password(&user.password)?;

        match db.prepare(r#"UPDATE "user" SET username = ?1, password = ?2 WHERE email = ?3"#) {
            Ok(rows) => {
                let user = map_user(rows, &[user.username, user.password, user.email])?;

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

pub async fn update_favorites(State(state): ConnectionState, Json(user): Json<User>) -> Response {
    let res: Result<Response> = (|| {
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
