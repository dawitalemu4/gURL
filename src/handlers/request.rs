use axum::{
    extract::{Json, Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use miette::{Result, miette};

use crate::{
    handlers::{
        get_all_favorites_from_db, get_all_requests_from_db, map_requests, ConnectionState, PathParams
    },
    models::{request::Request, serialize_bool_for_db},
};

pub async fn get_all_requests(state: ConnectionState, Path(path): Path<PathParams>) -> Response {
    let res: Result<Response> = (async || {
        let requests = get_all_requests_from_db(state, Path(path)).await;

        match requests {
            Ok(requests) => {
                if requests.is_empty() {
                    Ok((
                        StatusCode::NOT_FOUND,
                        "No requests found from this user email",
                    )
                        .into_response())
                } else {
                    Ok((StatusCode::OK, Json(requests)).into_response())
                }
            }
            Err(e) => Ok((
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Server error: {e}"),
            )
                .into_response()),
        }
    })()
    .await;

    match res {
        Ok(res) => res,
        Err(e) => panic!("{e}"),
    }
}

pub async fn get_all_favorite_requests(
    state: ConnectionState,
    Path(path): Path<PathParams>,
) -> Response {
    let res: Result<Response> = (async || {
        let favorites = get_all_favorites_from_db(state, Path(path)).await;

        match favorites {
            Ok(favorites) => {
                if favorites.is_empty() {
                    Ok((
                        StatusCode::NOT_FOUND,
                        "No favorite requests found from this user email",
                    )
                        .into_response())
                } else {
                    Ok((StatusCode::OK, Json(favorites)).into_response())
                }
            }
            Err(e) => Ok((
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Server error: {e}"),
            )
                .into_response()),
        }
    })()
    .await;

    match res {
        Ok(res) => res,
        Err(e) => panic!("{e}"),
    }
}

pub async fn create_request(
    State(state): ConnectionState,
    Path(path): Path<PathParams>,
    Json(request): Json<Request>,
) -> Response {
    let res: Result<Response> = (|| {
        let email = path.email.unwrap_or("anon".to_string());
        let db = state
            .lock()
            .map_err(|e| miette!("Global db can't block current thread {e}"))?;

        match db.prepare(r#"
            INSERT INTO request (user_email, url, method, metadata, payload, status, date, service, proto_file, hidden) 
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10) RETURNING *
        "#) {
            Ok(rows) => {
                let request = map_requests(rows, &[
                    email,
                    request.method.unwrap_or_default(),
                    request.status.unwrap_or_default(),
                    request.date,
                    serialize_bool_for_db(request.hidden).to_string()
                ])?[0].clone();

                if request.id == 0 {
                    Ok((
                            StatusCode::NOT_FOUND
                    )
                        .into_response())
                } else {
                    Ok((StatusCode::OK, Json(request)).into_response())
                }
            }
            Err(e) => Ok((StatusCode::INTERNAL_SERVER_ERROR, format!("Server Error: {e}")).into_response())
        }
    })();

    match res {
        Ok(res) => res,
        Err(e) => panic!("{e}"),
    }
}

pub async fn hide_request(State(state): ConnectionState, Path(path): Path<PathParams>) -> Response {
    let res: Result<Response> = (|| {
        let email = path.email.unwrap_or("anon".to_string());
        let request_id = path.request_id.expect("Missing request id");
        let db = state
            .lock()
            .map_err(|e| miette!("Global db can't block current thread {e}"))?;

        match db.prepare(
            "UPDATE request SET hidden = true WHERE user_email = ?1 AND id = ?2 RETURNING *",
        ) {
            Ok(rows) => {
                let request = map_requests(rows, &[email, request_id])?[0].clone();
                if request.hidden {
                    Ok((StatusCode::OK).into_response())
                } else {
                    Ok((StatusCode::NOT_FOUND).into_response())
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
