use std::sync::{Arc, Mutex};

use axum::{
    Router,
    routing::{delete, get, patch, post, put},
};
use tower_http::services::{ServeDir, ServeFile};

use crate::handlers::*;

pub fn init_router(global_db: Arc<Mutex<rusqlite::Connection>>) -> Router {
    Router::new()
        // User routes
        .route("/api/user/auth", post(get_user))
        .route("/api/user/new", post(create_user))
        .route("/api/user/update", put(update_user))
        .route("/api/user/update", delete(delete_user))
        .route("/api/user/favorites", patch(update_favorites))
        // Request routes
        .route("/api/request/all/{email}", get(get_all_requests))
        .route(
            "/api/request/favorites/{email}",
            get(get_all_favorite_requests),
        )
        .route("/api/request/new/{email}", post(create_request))
        .route("/api/request/delete/{email}/{req_id}", post(hide_request))
        // Template routes
        .route("/", get(render_page))
        .route("/login", get(render_page))
        .route("/signup", get(render_page))
        .route("/profile", get(render_page))
        .route("/handle/navbar/{page}/{token}", get(render_navbar))
        .route("/handle/username/{token}", get(render_username))
        .route("/handle/shortcut/{token}", get(render_home_shortcuts))
        .route("/handle/request/new/{email}", get(render_new_request))
        .route("/handle/request/history/{email}", get(render_history_list))
        .route(
            "/handle/request/favorites/{email}",
            get(render_favorites_list),
        )
        .route("/grpc/request", post(execute_grpcurl_request))
        .route("/handle/login/{token}", get(render_login))
        .route("/handle/signup/{token}", get(render_signup))
        .route("/handle/profile/info/{token}", get(render_profile_info))
        .route("/handle/profile/update/{token}", get(render_profile_update))
        .route(
            "/handle/profile/delete/{deleted}",
            get(render_profile_delete),
        )
        .nest_service("/public", ServeDir::new("src/views/public"))
        .nest_service("/robots.txt", ServeFile::new("src/views/public/robots.txt"))
        .nest_service("/css", ServeDir::new("src/views/css"))
        .nest_service("/js", ServeDir::new("src/views/js"))
        // Healtcheck route
        .route("/api/healthcheck", get("gURL is healthy"))
        .with_state(global_db)
}
