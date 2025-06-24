use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use miette::{Result, miette};

use crate::handlers::{ConnectionState, PathParams, map_requests};

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

    for id in ids {

        match db
            .prepare("SELECT * FROM request WHERE user_email = $1 AND id = $2 AND hidden = false ORDER BY id DESC")
            {
                Ok(rows) => {
                    let res = map_requests(rows, &[email, id])?;
                    favorite_requests.push(res);
                }
            }
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
        Err(e) => Ok((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(format!("Server Error: {e}")),
        )
            .into_response()),
    }
}
