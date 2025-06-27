use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use std::fmt;
use validator::Validate;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum HttpMethod {
    GET,
    POST,
    UPDATE,
    PUT,
    PATCH,
    DELETE,
}

impl fmt::Display for HttpMethod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HttpMethod::GET => write!(f, "GET"),
            HttpMethod::POST => write!(f, "POST"),
            HttpMethod::UPDATE => write!(f, "UPDATE"),
            HttpMethod::PUT => write!(f, "PUT"),
            HttpMethod::PATCH => write!(f, "PATCH"),
            HttpMethod::DELETE => write!(f, "DELETE"),
        }
    }
}

impl From<String> for HttpMethod {
    fn from(s: String) -> Self {
        match s.to_uppercase().as_str() {
            "GET" => HttpMethod::GET,
            "POST" => HttpMethod::POST,
            "PUT" => HttpMethod::PUT,
            "DELETE" => HttpMethod::DELETE,
            "PATCH" => HttpMethod::PATCH,
            _ => unimplemented!("Method is not supported"),
        }
    }
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
pub struct Request {
    pub id: i32,
    #[validate(email)]
    pub user_email: Option<String>,
    #[validate(url)]
    pub url: String,
    pub method: HttpMethod,
    pub origin: Option<String>,
    pub headers: Option<String>,
    pub body: Option<String>,
    #[validate(length(min = 1))]
    pub status: String,
    #[validate(length(min = 1))]
    pub date: String,
    pub hidden: bool,
}
