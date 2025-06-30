use askama::Template;
use axum::{
    extract::{Path, Request, State},
    http::StatusCode,
    response::{Html, IntoResponse, Response},
};
use miette::{Result, miette};

use crate::{
    handlers::{ConnectionState, PathParams, get_all_requests},
    parse_jwt,
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
            "/" => "index".to_string(),
            screen => screen.to_string(),
        };
        let index_active = match screen.as_str() {
            "index" => "navbar-active".to_string(),
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
        let page = path.page.unwrap_or("index".to_string());

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
            Ok((StatusCode::OK, Html("<p>$  incorrect credentials</p>")).into_response())
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
    let token_string = path.token.unwrap_or("null".to_string());

    match parse_jwt(&token_string) {
        Err(e) if e.to_string() == blank_token_error().to_string() => {
            Html("<p>$  invalid input</p>".to_string()).into_response()
        }
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("$  Server Error: {}", e),
        )
            .into_response(),
        Ok(Some(user)) => Html(format!(
            "<p>$  account updated! username: {}, email: {}, password: {}</p>",
            user.username, user.email, user.password
        ))
        .into_response(),
        Ok(None) => Html("<p>$  invalid input</p>".to_string()).into_response(),
    }
}

pub async fn render_profile_delete(Path(path): Path<PathParams>) -> Response {
    let deleted = path.deleted.unwrap_or("false".to_string());

    if deleted != "true" {
        Html("<p>$  invalid token, try to log back in</p>".to_string()).into_response()
    } else {
        Html("<p>$  deleting account</p>".to_string()).into_response()
    }
}

pub async fn render_home_shortcuts(Path(path): Path<PathParams>) -> Response {
    let token_string = path.token.unwrap_or("null".to_string());

    match parse_jwt(&token_string) {
        Ok(None) | Err(_) => Html(
            r#"
                <div><kbd>ctrl</kbd> + <kbd>alt</kbd> + <kbd>l</kbd> - login page</div>
                <div><kbd>ctrl</kbd> + <kbd>alt</kbd> + <kbd>s</kbd> - signup page</div>
            "#
            .to_string(),
        )
        .into_response(),
        Ok(Some(_)) => Html(
            r#"
                <div><kbd>ctrl</kbd> + <kbd>alt</kbd> + <kbd>p</kbd> - profile page</div>
                <div><kbd>ctrl</kbd> + <kbd>alt</kbd> + <kbd>l</kbd> - logout</div>
            "#
            .to_string(),
        )
        .into_response(),
    }
}

pub async fn render_new_request(Path(path): Path<PathParams>) -> Response {
    let email = path.email.unwrap_or("anon".to_string());

    let html = format!(
        r##"
        <form id="new-request"
            hx-post="/curl/request"
            hx-target="#request-response"
            hx-swap="innerHTML"
            hx-ext="json-enc"
            hx-on::before-request="loading()"
            hx-on::after-request="formatResponse()"
        >
            $  curl -X <select name="method" autofocus required>
                <option value="GET">GET</option>
                <option value="POST">POST</option>
                <option value="PUT">PUT</option>
                <option value="PATCH">PATCH</option>
                <option value="DELETE">DELETE</option>
            </select> \\ <br />
            -H '<input name="headers" type="text" placeholder="headers" />' \\ <br />
            -H '<input name="origin" type="text" placeholder="origin" />' \\ <br />
            -d '<textarea name="body" type="text" placeholder="body"></textarea>' \\ <br />
            <input name="url" type="text" placeholder="url" required />
            <input name="user_email" value="{email}" hidden />
            <input type="submit" hidden />
        </form>
        <div id="request-response "></div>
        "##
    );

    Html(html).into_response()
}

pub async fn render_history_list(
    State(state): State<ConnectionState>,
    Path(path): Path<PathParams>,
) -> Response {
    let res: Result<Response> = (|| {
        let email = path.email.unwrap_or("anon".to_string());
        let requests = get_all_requests(state, path);

        let status_colors = std::collections::HashMap::from([
            ("1", "green"),
            ("2", "green"),
            ("3", "yellow"),
            ("4", "red"),
            ("5", "orange"),
        ]);

        let mut html_history_list = String::new();

        for (i, request) in requests.iter().enumerate() {
            let date_ms: u64 = request.date.parse().unwrap_or(0);
            let formatted_date = format_duration(Duration::from_millis(date_ms));

            let status_color = status_colors
                .get(
                    request
                        .status
                        .chars()
                        .next()
                        .unwrap_or('5')
                        .to_string()
                        .as_str(),
                )
                .unwrap_or(&"orange");

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
                        <input type="hidden" name="url" value="{}" />
                        <input type="hidden" name="headers" value="{}" />
                        <input type="hidden" name="origin" value="{}" />
                        <textarea name="body" hidden>{}</textarea>
                    </div>
                    "#,
                i + 1,
                request.id,
                status_color,
                request.status,
                request.method,
                request.url,
                formatted_date,
                request.url,
                request.metadata.as_ref().unwrap_or(&String::new()),
                request.metadata.as_ref().unwrap_or(&String::new()), // Assuming origin is in metadata
                request.payload.as_ref().unwrap_or(&String::new())
            ));
        }

        Ok(Html(html_history_list).into_response())
    })();

    match res {
        Ok(res) => res,
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Server error: {e}"),
        )
            .into_response(),
    }
}

pub async fn render_favorites_list(Path(path): Path<PathParams>) -> Response {
    let res: Result<Response> = (|| {
        let email = path.email.unwrap_or("anon".to_string());
        let db = state
            .lock()
            .map_err(|e| miette!("Global db can't block current thread {e}"))?;

        // Get favorites from user table
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

        if favorites.is_empty() {
            return Ok(Html(
                r#"<br /><p style="margin-left:15px;">$  no favorites</p>"#.to_string(),
            )
            .into_response());
        }

        let mut favorite_requests = Vec::new();

        for favorite in favorites {
            let rows = match db.prepare(
                r#"SELECT * FROM request WHERE user_email = ?1 AND id = ?2 AND hidden = false ORDER BY id DESC"#,
            ) {
                Ok(rows) => rows,
                Err(e) => {
                    return Ok((
                        StatusCode::INTERNAL_SERVER_ERROR,
                        format!("Server Error: {e}"),
                    ).into_response());
                }
            };

            let requests = map_requests(rows, &[email.clone(), favorite])?;
            if !requests.is_empty() {
                favorite_requests.push(requests[0].clone());
            }
        }

        let status_colors = std::collections::HashMap::from([
            ("1", "green"),
            ("2", "green"),
            ("3", "yellow"),
            ("4", "red"),
            ("5", "orange"),
        ]);

        let mut html_favorites_list = String::new();

        for (i, request) in favorite_requests.iter().enumerate() {
            let date_ms: u64 = request.date.parse().unwrap_or(0);
            let formatted_date = format_duration(Duration::from_millis(date_ms));

            let status_color = status_colors
                .get(
                    request
                        .status
                        .chars()
                        .next()
                        .unwrap_or('5')
                        .to_string()
                        .as_str(),
                )
                .unwrap_or(&"orange");

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
                        <input type="hidden" name="url" value="{}" />
                        <input type="hidden" name="headers" value="{}" />
                        <input type="hidden" name="origin" value="{}" />
                        <textarea name="body" hidden>{}</textarea>
                    </div>
                    "#,
                i + 1,
                request.id,
                status_color,
                request.status,
                request.method,
                request.url,
                formatted_date,
                request.url,
                request.metadata.as_ref().unwrap_or(&String::new()),
                request.metadata.as_ref().unwrap_or(&String::new()),
                request.payload.as_ref().unwrap_or(&String::new())
            ));
        }

        Ok(Html(html_favorites_list).into_response())
    })();

    match res {
        Ok(res) => res,
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Server error: {e}"),
        )
            .into_response(),
    }
}
