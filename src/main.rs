use std::sync::{Arc, Mutex};

pub mod handlers;
pub mod models;
pub mod utils;

pub use handlers::*;
pub use utils::*;

use axum::{
    Router,
    routing::{delete, get, patch, post, put},
};
use miette::{Result, miette};
use tokio::net::TcpListener;
use tower_http::services::{ServeDir, ServeFile};

#[tokio::main]
async fn main() -> Result<()> {
    let (db, port) = (db(true)?, env()?.port);
    let global_db = Arc::new(Mutex::new(db));

    let router = Router::new()
        // User routes
        .route("/api/user/auth", post(get_user))
        .route("/api/user/new", post(create_user))
        .route("/api/user/update", put(update_user))
        .route("/api/user/update", delete(delete_user))
        .route("/api/user/favorites", patch(update_favorites))
        // Request routes
        .route("/api/request/all/:email", get(get_all_requests))
        .route(
            "/api/request/favorites/:email",
            get(get_all_favorite_requests),
        )
        .route("/api/request/new/:email", post(create_request))
        .route("/api/request/delete/:email/:req_id", post(hide_request))
        // Template routes
        .route("/handle/navbar/:page/:token", get(render_navbar))
        .route("/handle/username/:token", get(render_username))
        .route("/handle/shortcut/:token", get(render_home_shortcuts))
        .route("/handle/request/new/:email", get(render_new_request))
        .route("/handle/request/history/:email", get(render_history_list))
        .route(
            "/handle/request/favorites/:email",
            get(render_favorites_list),
        )
        .route("/grpc/request", post(execute_grpcurl_request))
        .route("/handle/login/:token", get(render_login))
        .route("/handle/signup/:token", get(render_signup))
        .route("/handle/profile/info/:token", get(render_profile_info))
        .route("/handle/profile/update/:token", get(render_profile_update))
        .route(
            "/handle/profile/delete/:deleted",
            get(render_profile_delete),
        )
        .nest_service("/public", ServeDir::new("views/public"))
        .nest_service("/robots.txt", ServeFile::new("views/public/robots.txt"))
        .nest_service("/css", ServeDir::new("views/css"))
        .nest_service("/js", ServeDir::new("views/js"))
        // Healtcheck route
        .route("/healthcheck", get("gURL is healthy"))
        .with_state(global_db);

    let listener = TcpListener::bind(format!("127.0.0.1:{port}"))
        .await
        .map_err(|e| miette!("Tokio unable to listen on port {port}: {e}"))?;

    axum::serve(listener, router)
        .await
        .map_err(|e| miette!("Axum unable to serve gURL router: {e}"))?;

    Ok(())
}
