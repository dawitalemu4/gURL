use askama::Template;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{Html, IntoResponse},
};
use miette::{Result, miette};

use crate::handlers::{ConnectionState, PathParams};

// Template for navbar profile section
#[derive(Template)]
#[template(path = "components.html", block = "navbar_profile")]
pub struct NavbarProfileTemplate {
    pub screen: String,
    pub token: &'static str,
}

#[derive(Template)]
#[template(path = "components.html", block = "username_display")]
pub struct UsernameTemplate {
    pub username: String,
}

#[derive(Template)]
#[template(path = "components.html", block = "index_shortcuts")]
pub struct HomeShortcutsTemplate;

#[derive(Template)]
#[template(path = "components.html", block = "new_request_form")]
pub struct NewRequestTemplate {
    pub email: Option<String>,
}

#[derive(Template)]
#[template(path = "components.html", block = "history_list")]
pub struct HistoryListTemplate {
    pub requests: Vec<RequestItem>,
}

#[derive(Template)]
#[template(path = "components.html", block = "favorites_list")]
pub struct FavoritesListTemplate {
    pub requests: Vec<RequestItem>,
}

#[derive(Clone)]
pub struct RequestItem {
    pub id: String,
    pub method: String,
    pub url: String,
    pub headers: String,
    pub origin: String,
    pub body: String,
}

pub async fn render_navbar(
    State(_state): ConnectionState,
    Path(path): Path<PathParams>,
) -> impl IntoResponse {
    let screen = path.page.unwrap_or_else(|| "index".to_string());
    match (NavbarProfileTemplate {
        screen,
        token: path.token,
    })
    .render()
    {
        Ok(html) => Html(html).into_response(),
        Err(e) => {
            eprintln!("Template error in render_navbar: {e:?}");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Html("Internal server error".to_string()),
            )
                .into_response()
        }
    }
}

pub async fn render_username(
    State(_state): ConnectionState,
    Path(_path): Path<PathParams>,
) -> impl IntoResponse {
    match (UsernameTemplate {
        username: "User".to_string(),
    })
    .render()
    {
        Ok(html) => Html(html).into_response(),
        Err(e) => {
            eprintln!("Template error in render_username: {e:?}");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Html("Internal server error".to_string()),
            )
                .into_response()
        }
    }
}

pub async fn render_home_shortcuts(
    State(_state): ConnectionState,
    Path(_path): Path<PathParams>,
) -> impl IntoResponse {
    match (HomeShortcutsTemplate {}).render() {
        Ok(html) => Html(html).into_response(),
        Err(e) => {
            eprintln!("Template error in render_home_shortcuts: {e:?}");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Html("Internal server error".to_string()),
            )
                .into_response()
        }
    }
}

pub async fn render_new_request(
    State(_state): ConnectionState,
    Path(path): Path<PathParams>,
) -> impl IntoResponse {
    let email = path.page;
    match (NewRequestTemplate { email }).render() {
        Ok(html) => Html(html).into_response(),
        Err(e) => {
            eprintln!("Template error in render_new_request: {e:?}");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Html("Internal server error".to_string()),
            )
                .into_response()
        }
    }
}

pub async fn render_history_list(
    State(_state): ConnectionState,
    Path(path): Path<PathParams>,
) -> impl IntoResponse {
    let email = path.page;
    let requests = Vec::new();
    match (HistoryListTemplate { requests }).render() {
        Ok(html) => Html(html).into_response(),
        Err(e) => {
            eprintln!("Template error in render_history_list: {e:?}");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Html("Internal server error".to_string()),
            )
                .into_response()
        }
    }
}

pub async fn render_favorites_list(
    State(_state): ConnectionState,
    Path(path): Path<PathParams>,
) -> impl IntoResponse {
    let email = path.page;
    let requests = Vec::new();
    match (FavoritesListTemplate { requests }).render() {
        Ok(html) => Html(html).into_response(),
        Err(e) => {
            eprintln!("Template error in render_favorites_list: {e:?}");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Html("Internal server error".to_string()),
            )
                .into_response()
        }
    }
}

pub async fn execute_curl_request(
    State(_state): ConnectionState,
    axum::Json(_body): axum::Json<serde_json::Value>,
) -> impl IntoResponse {
    Html("<div>Request executed</div>")
}

pub async fn render_login(
    State(_state): ConnectionState,
    Path(_path): Path<PathParams>,
) -> impl IntoResponse {
    Html(
        r#"
        <div id="login-container">
            <form id="login-form">
                <br />
                <div>
                    <label for="email">$  email: </label>
                    <input id="login-email" type="text" name="email" placeholder="_" autofocus required />
                </div>
                <br />
                <div>
                    <label for="password">$  password: </label>
                    <input id="login-password" type="text" name="password" placeholder="_" required />
                </div>
                <input type="submit" hidden />
                <div id="login-response"></div>
                <div id="login-timer"></div>
            </form>
        </div>
    "#,
    )
}

pub async fn render_signup(
    State(_state): ConnectionState,
    Path(_path): Path<PathParams>,
) -> impl IntoResponse {
    Html(
        r#"
        <div id="signup-container">
            <form id="signup-form">
                <br />
                <div>
                    <label for="username">$  username: </label>
                    <input id="signup-username" type="text" name="username" placeholder="_" autofocus required />
                </div>
                <br />
                <div>
                    <label for="email">$  email: </label>
                    <input id="signup-email" type="text" name="email" placeholder="_" required />
                </div>
                <br />
                <div>
                    <label for="password">$  password: </label>
                    <input id="signup-password" type="text" name="password" placeholder="_" required />
                </div>
                <input type="submit" hidden />
                <div id="signup-response"></div>
                <div id="signup-timer"></div>
            </form>
        </div>
    "#,
    )
}

pub async fn render_profile_info(
    State(_state): ConnectionState,
    Path(_path): Path<PathParams>,
) -> impl IntoResponse {
    Html(
        r#"
        <div id="profile-container">
            <p>$  Profile Info</p>
            <div id="profile-info"></div>
            <br />
            <p>$  Update Profile Info</p>
            <form id="profile-form">
                <br />
                <div>
                    <label for="username">$  username: </label>
                    <input id="profile-username" type="text" name="username" placeholder="_" autofocus required />
                </div>
                <br />
                <div>
                    <label for="password">$  password: </label>
                    <input id="profile-password" type="text" name="password" placeholder="_" required />
                </div>
                <input type="submit" hidden />
                <br />
                <p onclick="deleteProfile()">$  Delete Profile</p>
                <div id="profile-response"></div>
                <div id="profile-timer"></div>
            </form>
        </div>
    "#,
    )
}

pub async fn render_profile_update(
    State(_state): ConnectionState,
    Path(_path): Path<PathParams>,
) -> impl IntoResponse {
    Html(
        r#"
        <div id="profile-container">
            <p>$  Profile Info</p>
            <div id="profile-info"></div>
            <br />
            <p>$  Update Profile Info</p>
            <form id="profile-form">
                <br />
                <div>
                    <label for="username">$  username: </label>
                    <input id="profile-username" type="text" name="username" placeholder="_" autofocus required />
                </div>
                <br />
                <div>
                    <label for="password">$  password: </label>
                    <input id="profile-password" type="text" name="password" placeholder="_" required />
                </div>
                <input type="submit" hidden />
                <br />
                <p onclick="deleteProfile()">$  Delete Profile</p>
                <div id="profile-response"></div>
                <div id="profile-timer"></div>
            </form>
        </div>
    "#,
    )
}

pub async fn render_profile_delete(
    State(_state): ConnectionState,
    Path(_path): Path<PathParams>,
) -> impl IntoResponse {
    Html(
        r#"
        <div id="profile-container">
            <p>$  Profile Info</p>
            <div id="profile-info"></div>
            <br />
            <p>$  Update Profile Info</p>
            <form id="profile-form">
                <br />
                <div>
                    <label for="username">$  username: </label>
                    <input id="profile-username" type="text" name="username" placeholder="_" autofocus required />
                </div>
                <br />
                <div>
                    <label for="password">$  password: </label>
                    <input id="profile-password" type="text" name="password" placeholder="_" required />
                </div>
                <input type="submit" hidden />
                <br />
                <p onclick="deleteProfile()">$  Delete Profile</p>
                <div id="profile-response"></div>
                <div id="profile-timer"></div>
            </form>
        </div>
    "#,
    )
}
