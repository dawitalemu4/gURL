use askama::Template;
use axum::{
    http::StatusCode,
    response::{Html, IntoResponse, Response},
};

struct HtmlTemplate<T>(T);

impl<T> IntoResponse for HtmlTemplate<T>
where
    T: Template,
{
    fn into_response(self) -> Response {
        match self.0.render() {
            Ok(html) => Html(html).into_response(),
            Err(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Internal server error: {err}"),
            )
                .into_response(),
        }
    }
}

#[derive(Template)]
#[template(path = "index.html")]
pub struct RenderTemplate {
    pub screen: Option<String>,
    pub token: Option<String>,
    pub email: Option<String>,
    pub deleted: Option<String>,
}
