use askama::Template;
use axum::{
    extract::{Path, Request},
    http::StatusCode,
    response::{Html, IntoResponse, Response},
};
use miette::{Result, miette};

use crate::{
    ConnectionState, PathParams, get_all_favorites_from_db, get_all_requests_from_db,
    get_status_color, humanize_date, parse_jwt,
};

#[derive(Template, Debug)]
#[template(path = "index.html")]
pub struct IndexTemplate {
    screen: String,
    index_active: String,
}

pub async fn render_page(request: Request) -> Response {
    let res: Result<Response> = (|| {
        let screen = match request.uri().to_string().as_str() {
            "/" => "home".to_string(),
            screen => screen.strip_prefix("/").unwrap_or(screen).to_string(),
        };
        let index_active = match screen.as_str() {
            "home" => "navbar-active".to_string(),
            _ => "".to_string(),
        };

        let template = IndexTemplate {
            screen,
            index_active,
        };

        let html = template
            .render()
            .map_err(|e| miette!("Server Error: {e}"))?;

        Ok((StatusCode::OK, Html(html)).into_response())
    })();

    match res {
        Ok(res) => res,
        Err(e) => panic!("Server Error: {e}"),
    }
}

pub async fn render_navbar(Path(path): Path<PathParams>) -> Response {
    let res: Result<Response> = (|| {
        let token = path.token.unwrap_or("null".to_string());
        let page = path.page.unwrap_or("home".to_string());

        let mut pages = std::collections::HashMap::new();
        pages.insert("login", "");
        pages.insert("signup", "");
        pages.insert("profile", "");
        pages.insert(&page, "navbar-active");

        if token == "null".to_string() {
            let html = format!(
                r#"
                    <a id="{}" href="/login">login /</a>
                    <a id="{}" href="/signup">/ signup</a>
                "#,
                pages.get("login").unwrap_or(&""),
                pages.get("signup").unwrap_or(&"")
            );

            Ok((StatusCode::OK, Html(html)).into_response())
        } else {
            let user = parse_jwt(&token)?;
            let html = format!(
                r#"
                    <a id="{}" href="/profile">{} /</a>
                    <a onclick="logout();">/ logout</a>
                "#,
                pages.get("profile").unwrap_or(&""),
                user.username
            );

            Ok((StatusCode::OK, Html(html)).into_response())
        }
    })();

    match res {
        Ok(res) => res,
        Err(e) => panic!("Server Error: {e}"),
    }
}

pub async fn render_username(Path(path): Path<PathParams>) -> Response {
    let res: Result<Response> = (|| {
        let token = path.token.unwrap_or("null".to_string());

        if token == "null".to_string() {
            let html = "<p>$  hello anon! Signup or login to save your favorite requests and organize your request history in your own profiles</p>";
            Ok((StatusCode::OK, Html(html)).into_response())
        } else {
            let user = parse_jwt(&token)?;
            Ok((
                StatusCode::OK,
                format!("<p>$  hello {}!</p>", user.username),
            )
                .into_response())
        }
    })();

    match res {
        Ok(res) => res,
        Err(e) => panic!("Server Error: {e}"),
    }
}

pub async fn render_login(Path(path): Path<PathParams>) -> Response {
    let res: Result<Response> = (|| {
        let token = path.token.unwrap_or("null".to_string());

        if token == "null".to_string() {
            Ok((
                StatusCode::OK,
                Html("<p>$  incorrect credentials or user doesn't exist</p>"),
            )
                .into_response())
        } else {
            let user = parse_jwt(&token)?;
            Ok((
                StatusCode::OK,
                Html(format!("<p>$  welcome back {}!</p>", user.username)),
            )
                .into_response())
        }
    })();

    match res {
        Ok(res) => res,
        Err(e) => panic!("Server Error: {e}"),
    }
}

pub async fn render_signup(Path(path): Path<PathParams>) -> Response {
    let res: Result<Response> = (|| {
        let token = path.token.unwrap_or("null".to_string());

        if token == "null".to_string() {
            Ok((StatusCode::OK, Html("<p>$  invalid input</p>".to_string())).into_response())
        } else {
            let user = parse_jwt(&token)?;

            Ok((
                StatusCode::OK,
                Html(format!(
                    "<p>$  account created! username: {}, email: {}</p>",
                    user.username, user.email
                )),
            )
                .into_response())
        }
    })();

    match res {
        Ok(res) => res,
        Err(e) => panic!("Server Error: {e}"),
    }
}

pub async fn render_profile_info(Path(path): Path<PathParams>) -> Response {
    let res: Result<Response> = (|| {
        let token = path.token.unwrap_or("null".to_string());

        if token == "null".to_string() {
            Ok((StatusCode::OK, Html("<p>$  invalid token</p>")).into_response())
        } else {
            let user = parse_jwt(&token)?;
            let user_since = humanize_date(user.date)?;

            Ok((
                StatusCode::OK,
                Html(format!(
                    "<p>$  username: {}, email: {}, user since {}</p>",
                    user.username, user.email, user_since
                )),
            )
                .into_response())
        }
    })();

    match res {
        Ok(res) => res,
        Err(e) => panic!("Server Error: {e}"),
    }
}

pub async fn render_profile_update(Path(path): Path<PathParams>) -> Response {
    let res: Result<Response> = (|| {
        let token = path.token.unwrap_or("null".to_string());

        if token == "null".to_string() {
            Ok((StatusCode::OK, Html("<p>$  invalid input</p>")).into_response())
        } else {
            let user = parse_jwt(&token)?;

            Ok((
                StatusCode::OK,
                Html(format!(
                    "<p>$  account updated! username: {}, email: {}, password: {}</p>",
                    user.username, user.email, user.password
                )),
            )
                .into_response())
        }
    })();

    match res {
        Ok(res) => res,
        Err(e) => panic!("Server Error: {e}"),
    }
}

pub async fn render_profile_delete(Path(path): Path<PathParams>) -> Response {
    let deleted = path.deleted.unwrap_or(false);

    if deleted {
        Html("<p>$  invalid token, try to log back in</p>".to_string()).into_response()
    } else {
        Html("<p>$  deleting account</p>".to_string()).into_response()
    }
}

pub async fn render_home_shortcuts(Path(path): Path<PathParams>) -> Response {
    let res: Result<Response> = (|| {
        let token = path.token.unwrap_or("null".to_string());

        if token == "null".to_string() {
            Ok((
                StatusCode::OK,
                Html(
                    r#"
                    <div><kbd>ctrl</kbd> + <kbd>alt</kbd> + <kbd>l</kbd> - login page</div>
                    <div><kbd>ctrl</kbd> + <kbd>alt</kbd> + <kbd>s</kbd> - signup page</div>
                "#,
                ),
            )
                .into_response())
        } else {
            Ok((
                StatusCode::OK,
                Html(
                    r#"
                    <div><kbd>ctrl</kbd> + <kbd>alt</kbd> + <kbd>p</kbd> - profile page</div>
                    <div><kbd>ctrl</kbd> + <kbd>alt</kbd> + <kbd>l</kbd> - logout</div>
                "#,
                ),
            )
                .into_response())
        }
    })();

    match res {
        Ok(res) => res,
        Err(e) => panic!("Server Error: {e}"),
    }
}

pub async fn render_new_request(Path(path): Path<PathParams>) -> Response {
    let email = path.email.unwrap_or("anon".to_string());

    let html = format!(
        r##"
        <form id="new-request"
            hx-post="/grpcurl/request"
            hx-target="#request-response"
            hx-swap="innerHTML"
            hx-ext="json-enc"
            hx-on::before-request="loading()"
            hx-on::after-request="formatResponse()"
        >
            $  grpcurl <textarea name="command" type="text" placeholder="command" autofocus></textarea>
            <input name="user_email" value="{email}" hidden />
            <input type="submit" value="execute" />
        </form>
        <div id="request-response"></div>
        "##
    );

    Html(html).into_response()
}

pub async fn render_history_list(state: ConnectionState, Path(path): Path<PathParams>) -> Response {
    let res: Result<Response> = (async || {
        let requests = get_all_requests_from_db(state, Path(path)).await?;
        let mut html_history_list = String::new();

        if requests.is_empty() {
            return Ok((
                StatusCode::OK,
                Html(r#"<br /><p style="margin-left:15px;">$  no history</p"#),
            )
                .into_response());
        }

        for (i, request) in requests.iter().enumerate() {
            let date = humanize_date(Some(request.date.clone()))?;
            let status_color = get_status_color(&request.status);

            html_history_list.push_str(&format!(
                r#"
                    <div class="history-item" tabindex="{}" id="{}">
                        <div class="history-item-left-container">
                            <p style="color: {};font-size:18px;">{}</p>
                            <p>{}</p>
                        </div>
                        <div class="history-item-right-container">
                            <p><bdi>{}</bdi></p>
                            <p>{}</p>
                        </div>
                        <div class="added-favorite">added to favorites</div>
                        <div class="removed-favorite">removed from favorites</div>
                        <div class="not-loggedin">log in to save favorites</div>
                        <div class="deleted-item">deleted item</div>
                        <input type="hidden" name="command" value="{}" />
                    </div>
                    "#,
                i + 1,
                request.id,
                status_color,
                request.status.clone().unwrap_or_default(),
                request.method.clone().unwrap_or_default(),
                request.command,
                date,
                request.command,
            ));
        }

        Ok(Html(html_history_list).into_response())
    })()
    .await;

    match res {
        Ok(res) => res,
        Err(e) => panic!("Server Error: {e}"),
    }
}

pub async fn render_favorites_list(
    state: ConnectionState,
    Path(path): Path<PathParams>,
) -> Response {
    let res: Result<Response> = (async || {
        let favorites = get_all_favorites_from_db(state, Path(path)).await?;
        let mut html_favorites_list = String::new();

        if favorites.is_empty() {
            return Ok(Html(
                r#"<br /><p style="margin-left:15px;">$  no favorites</p>"#.to_string(),
            )
            .into_response());
        }

        for (i, request) in favorites.iter().enumerate() {
            let date = humanize_date(Some(request.date.clone()))?;
            let status_color = get_status_color(&request.status);

            html_favorites_list.push_str(&format!(
                r#"
                    <div class="favorites-item" tabindex="{}" id="{}">
                        <div class="favorites-item-left-container">
                            <p style="color: {};font-size:18px;">{}</p>
                            <p>{}</p>
                        </div>
                        <div class="favorites-item-right-container">
                            <p><bdi>{}</bdi></p>
                            <p>{}</p>
                        </div>
                        <div class="added-favorite">added to favorites</div>
                        <div class="removed-favorite">removed from favorites</div>
                        <div class="deleted-item">deleted item</div>
                        <input type="hidden" name="command" value="{}" />
                    </div>
                    "#,
                i + 1,
                request.id,
                status_color,
                request.status.clone().unwrap_or_default(),
                request.method.clone().unwrap_or_default(),
                request.command,
                date,
                request.command,
            ));
        }

        Ok(Html(html_favorites_list).into_response())
    })()
    .await;

    match res {
        Ok(res) => res,
        Err(e) => panic!("Server Error: {e}"),
    }
}
