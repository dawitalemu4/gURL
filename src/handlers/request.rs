use axum::{
    Json,
    extract::{self, Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use miette::{Result, miette};

use crate::handlers::{ConnectionState, PathParams, RequestBody, map_requests, map_single_value};
use crate::models::request::Request;

pub async fn get_all_requests(
    State(state): ConnectionState,
    Path(path): Path<PathParams>,
) -> impl IntoResponse {
    let res: Result<(StatusCode, Json<Vec<Request>>)> = (|| {
        let email = path.email.unwrap_or_default();
        let db = state
            .lock()
            .map_err(|e| miette!("Global db can't block current thread {e}"))?;

        match db.prepare(
            "SELECT * FROM request WHERE user_email = ?1 AND hidden = false ORDER BY id DESC",
        ) {
            Ok(rows) => {
                let res = map_requests(rows, &[email])?;

                if res.is_empty() {
                    Ok((StatusCode::NOT_FOUND, Json(vec![])))
                } else {
                    Ok((StatusCode::OK, Json(res)))
                }
            }
            Err(e) => Ok((StatusCode::INTERNAL_SERVER_ERROR, Json(vec![]))),
        }
    })();

    match res {
        Ok(response) => response.into_response(),
        Err(e) => {
            eprintln!("Error in get_all_requests: {e:?}");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json("Internal server error"),
            )
                .into_response()
        }
    }
}

pub async fn get_all_favorite_requests(
    State(state): ConnectionState,
    Path(path): Path<PathParams>,
) -> impl IntoResponse {
    let res: Result<(StatusCode, Json<Vec<Vec<Request>>>)> = (|| {
        let mut favorite_requests = Vec::new();
        let email = path.email.unwrap_or_default();
        let db = state
            .lock()
            .map_err(|e| miette!("Global db can't block current thread {e}"))?;

        let favorite_rows = db
            .prepare(r#"SELECT favorites FROM "user" WHERE email = ?1"#)
            .map_err(|e| miette!(e));
        if favorite_rows.is_err() {
            return Ok((StatusCode::INTERNAL_SERVER_ERROR, Json(vec![])));
        }

        let favorites = map_single_value(favorite_rows?, &[email.clone()], "favorite")?;
        for favorite in favorites {
            let rows = db
                .prepare("SELECT * FROM request WHERE user_email = ?1 AND id = ?2 AND hidden = false ORDER BY id DESC").map_err(|e| miette!(e));
            if rows.is_err() {
                return Ok((StatusCode::INTERNAL_SERVER_ERROR, Json(vec![])));
            }

            let res = map_requests(rows?, &[email.clone(), favorite])?;
            favorite_requests.push(res);
        }

        if favorite_requests.is_empty() {
            Ok((StatusCode::NOT_FOUND, Json(vec![])))
        } else {
            Ok((StatusCode::OK, Json(favorite_requests)))
        }
    })();

    match res {
        Ok(response) => response.into_response(),
        Err(e) => {
            eprintln!("Error in get_all_favorite_requests: {e:?}");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json("Internal server error"),
            )
                .into_response()
        }
    }
}

pub async fn create_request(
    State(state): ConnectionState,
    Path(path): Path<PathParams>,
    extract::Json(body): extract::Json<RequestBody>,
) -> impl IntoResponse {
    let res: Result<(StatusCode, Json<Vec<Request>>)> = (|| {
        let email = path.email.unwrap_or_default();
        let request = body.request.unwrap();
        let db = state
            .lock()
            .map_err(|e| miette!("Global db can't block current thread {e}"))?;

        match db
            .prepare("INSERT INTO request (user_email, url, method, origin, headers, body, status, date, hidden) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9) RETURNING *")
        {
            Ok(rows) => {
                let res = map_requests(rows, &[
                    email,
                    request.url,
                    request.method.to_string(),
                    request.origin.unwrap_or_default(),
                    request.headers.unwrap_or_default(),
                    request.body.unwrap_or_default(),
                    request.status,
                    request.date,
                    request.hidden.to_string()
                ])?;

                if res.is_empty() {
                    Ok((StatusCode::NOT_FOUND, Json(vec![])))
                } else {
                    Ok((StatusCode::OK, Json(res)))
                }
            }
            Err(e) => Ok((StatusCode::INTERNAL_SERVER_ERROR, Json(vec![]))),
        }
    })();

    match res {
        Ok(response) => response.into_response(),
        Err(e) => {
            eprintln!("Error in create_request: {e:?}");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json("Internal server error"),
            )
                .into_response()
        }
    }
}

pub async fn hide_request(
    State(state): ConnectionState,
    Path(path): Path<PathParams>,
) -> impl IntoResponse {
    let res: Result<(StatusCode, Json<Vec<Request>>)> = (|| {
        let email = path.email.unwrap_or_default();
        let request_id = path.request_id.unwrap_or_default();
        let db = state
            .lock()
            .map_err(|e| miette!("Global db can't block current thread {e}"))?;

        match db.prepare(
            "UPDATE request SET hidden = true WHERE user_email = ?1 AND id = ?2 RETURNING *",
        ) {
            Ok(rows) => {
                let res = map_requests(rows, &[email, request_id])?;
                if res.is_empty() {
                    Ok((StatusCode::NOT_FOUND, Json(vec![])))
                } else {
                    Ok((StatusCode::OK, Json(res)))
                }
            }
            Err(e) => Ok((StatusCode::INTERNAL_SERVER_ERROR, Json("Server Error: {e}"))),
        }
    })();

    match res {
        Ok(response) => response.into_response(),
        Err(e) => {
            eprintln!("Error in hide_request: {e:?}");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json("Internal server error"),
            )
                .into_response()
        }
    }
}
