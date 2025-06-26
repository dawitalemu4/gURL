use axum::{
    Json,
    extract::{Path, State, self},
    http::StatusCode,
    response::IntoResponse,
};
use miette::{Result, miette};

use crate::handlers::{ConnectionState, PathParams, RequestBody, map_single_value, map_requests};

pub fn get_all_requests(
    State(state): ConnectionState,
    Path(path): Path<PathParams>,
) -> Result<impl IntoResponse> {
    let email = path.email.unwrap_or_default();
    let db = state
        .lock()
        .map_err(|e| miette!("Global db can't block current thread {e}"))?;

    match db
        .prepare("SELECT * FROM request WHERE user_email = ?1 AND hidden = false ORDER BY id DESC")
    {
        Ok(rows) => {
            let res = map_requests(rows, &[email])?;

            if res.is_empty() {
                Ok((
                    StatusCode::NOT_FOUND,
                    Json("No requests found from this user email"),
                )
                    .into_response())
            } else {
                Ok((StatusCode::OK, Json(res)).into_response())
            }
        }
        Err(e) => Ok((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(format!("Server Error: {e}")),
        )
            .into_response()),
    }
}

pub fn get_all_favorite_requests(
    State(state): ConnectionState,
    Path(path): Path<PathParams>,
) -> Result<impl IntoResponse> {
    let mut favorite_requests = Vec::new();
    let email = path.email.unwrap_or_default();
    let db = state
        .lock()
        .map_err(|e| miette!("Global db can't block current thread {e}"))?;

    let favorite_rows = db.prepare(r#"SELECT favorites FROM "user" WHERE email = ?1"#).map_err(|e| miette!(e));
    if favorite_rows.is_err() {
        return Ok((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(format!("Server Error: {:#?}", favorite_rows.err())),
        )
            .into_response());
    }

    let favorites = map_single_value(favorite_rows?, &[email.clone()], "favorite")?;
    for favorite in favorites {
        let rows = db
            .prepare("SELECT * FROM request WHERE user_email = ?1 AND id = ?2 AND hidden = false ORDER BY id DESC").map_err(|e| miette!(e));
        if rows.is_err() {
            return Ok((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(format!("Server Error: {:#?}", rows.err())),
            )
                .into_response());
        }

        let res = map_requests(rows?, &[email.clone(), favorite])?;
        favorite_requests.push(res);
    }

    if favorite_requests.is_empty() {
        Ok((
            StatusCode::NOT_FOUND,
            Json("No requests found from this user email"),
        )
            .into_response())
    } else {
        Ok((StatusCode::OK, Json(favorite_requests)).into_response())
    }
}

pub fn create_request(
    State(state): ConnectionState,
    Path(path): Path<PathParams>,
    extract::Json(body): extract::Json<RequestBody>
    
) -> Result<impl IntoResponse> {
    let email = path.email.unwrap_or_default();
    let request = body.request.unwrap();
    let db = state
        .lock()
        .map_err(|e| miette!("Global db can't block current thread {e}"))?;

    match db
        .prepare("INSERT INTO request (user_email, url, method, origin, headers, body, status, date, hidden) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9) RETURNING *")
    {
        Ok(rows) => {
            let res = map_requests(rows, &[email, request.url, request.method, request.origin, request.headers, request.body, request.status, request.date, request.hidden])?;

            if res.is_empty() {
                Ok((
                    StatusCode::NOT_FOUND,
                    Json("No requests found from this user email"),
                )
                    .into_response())
            } else {
                Ok((StatusCode::OK, Json(res)).into_response())
            }
        }
        Err(e) => Ok((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(format!("Server Error: {e}")),
        )
            .into_response()),
    }
}
