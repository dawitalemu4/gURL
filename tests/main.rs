use std::sync::{Arc, Mutex};

use axum::{
    body::Body,
    http::{Request, Response, StatusCode},
};

use gURL::{db, init_router};

async fn test_axum_request(
    route: &str,
    method: &str,
    params: Option<&str>,
    body: Option<Body>,
) -> Response<Body> {
    let db = db(true, true)?;
    let global_db = Arc::new(Mutex::new(db));
    let router = init_router(global_db);

    router
        .oneshot(
            Request::builder()
                .uri(format!("/{route}?{}", params.unwrap_or_default()))
                .method(method)
                .body(body.unwrap_or(Body::empty()))
                .unwrap(),
        )
        .await
        .unwrap()
}

#[tokio::test]
async fn test_healthcheck_route() {
    let res = test_axum_request("healthcheck", "GET", None, None).await;

    assert_eq!(res.status(), StatusCode::OK)
}
