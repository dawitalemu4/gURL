use axum::{
    extract::{Json, Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use miette::{Result, miette};

use crate::handlers::{ConnectionState, PathParams, RequestBody, map_requests, map_single_value};

pub async fn get_all_requests(
    State(state): ConnectionState,
    Path(path): Path<PathParams>,
) -> Response {
    let res: Result<Response> = (|| {
        let email = path.email.unwrap_or("anon".to_string());
        let db = state
            .lock()
            .map_err(|e| miette!("Global db can't block current thread {e}"))?;

        match db.prepare(
            "SELECT * FROM request WHERE user_email = ?1 AND hidden = false ORDER BY id DESC",
        ) {
            Ok(rows) => {
                let requests = map_requests(rows, &[email])?;

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
    })();

    match res {
        Ok(res) => res,
        Err(e) => panic!("{e}"),
    }
}

pub async fn get_all_favorite_requests(
    State(state): ConnectionState,
    Path(path): Path<PathParams>,
) -> Response {
    let res: Result<Response> = (|| {
        let mut favorite_requests = Vec::new();
        let email = path.email.unwrap_or("anon".to_string());
        let db = state
            .lock()
            .map_err(|e| miette!("Global db can't block current thread {e}"))?;

        let favorite_rows = match db.prepare(r#"SELECT favorites FROM "user" WHERE email = ?1"#) {
            Ok(rows) => rows,
            Err(e) => {
                return Ok((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Server Error: {e}"),
                )
                    .into_response());
            }
        };

        let favorites = map_single_value(favorite_rows, &[email.clone()], "favorite")?;
        for favorite in favorites {
            let rows = match db.prepare(
                r#"
                SELECT * FROM request WHERE user_email = ?1 AND id = ?2 AND hidden = false 
                ORDER BY id DESC
            "#,
            ) {
                Ok(rows) => rows,
                Err(e) => {
                    return Ok((
                        StatusCode::INTERNAL_SERVER_ERROR,
                        format!("Server Error: {e}"),
                    )
                        .into_response());
                }
            };

            let res = map_requests(rows, &[email.clone(), favorite])?;
            favorite_requests.push(res[0].clone());
        }

        if favorite_requests.is_empty() {
            Ok((
                StatusCode::NOT_FOUND,
                "No favorite requests found from this user email",
            )
                .into_response())
        } else {
            Ok((StatusCode::OK, Json(favorite_requests)).into_response())
        }
    })();

    match res {
        Ok(res) => res,
        Err(e) => panic!("{e}"),
    }
}

pub async fn create_request(
    State(state): ConnectionState,
    Path(path): Path<PathParams>,
    Json(body): Json<RequestBody>,
) -> Response {
    let res: Result<Response> = (|| {
        let email = path.email.unwrap_or("anon".to_string());
        let request = body.request.expect("Cannot serialize Request from body");
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
                    request.url,
                    request.method.to_string(),
                    request.metadata.unwrap_or_default(),
                    request.payload.unwrap_or_default(),
                    request.status,
                    request.date,
                    request.service.unwrap_or_default(),
                    request.proto_file.unwrap_or_default(),
                    request.hidden.to_string()
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
