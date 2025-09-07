use std::sync::{Arc, Mutex};

use axum::{
    Router,
    routing::{delete, get, patch, post},
};
use tower_http::services::ServeDir;

use crate::handlers::*;

pub fn init_router(global_db: Arc<Mutex<rusqlite::Connection>>) -> Router {
    Router::new()
        // User routes
        .route("/api/user/auth", post(get_user))
        .route(
            "/api/user",
            post(create_user).put(update_user).delete(delete_user),
        )
        .route("/api/user/favorites", patch(update_favorites))
        // Request routes
        .route(
            "/api/request/{email}",
            get(get_all_requests).post(create_request),
        )
        .route(
            "/api/request/favorites/{email}",
            get(get_all_favorite_requests),
        )
        .route("/api/request/delete/{email}/{id}", delete(hide_request))
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
        .route("/grpcurl/request/{email}", post(execute_grpcurl_request))
        .route("/handle/login/{token}", get(render_login))
        .route("/handle/signup/{token}", get(render_signup))
        .route("/handle/profile/info/{token}", get(render_profile_info))
        .route("/handle/profile/update/{token}", get(render_profile_update))
        .route("/handle/profile/delete", get(render_profile_delete))
        .nest_service("/public", ServeDir::new("public"))
        // Healtcheck route
        .route("/api/healthcheck", get("gURL is healthy"))
        .with_state(global_db)
}
