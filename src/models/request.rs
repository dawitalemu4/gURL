use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use std::fmt;
use validator::Validate;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum GrpcMethod {
    Unary,
    ServerStreaming,
    ClientStreaming,
    BidirectionalStreaming,
    Custom(String),
}

impl fmt::Display for GrpcMethod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GrpcMethod::Unary => write!(f, "unary"),
            GrpcMethod::ServerStreaming => write!(f, "server-streaming"),
            GrpcMethod::ClientStreaming => write!(f, "client-streaming"),
            GrpcMethod::BidirectionalStreaming => write!(f, "bidi-streaming"),
            GrpcMethod::Custom(method) => write!(f, "{}", method),
        }
    }
}

impl From<String> for GrpcMethod {
    fn from(s: String) -> Self {
        match s.to_lowercase().as_str() {
            "unary" => GrpcMethod::Unary,
            "server-streaming" => GrpcMethod::ServerStreaming,
            "client-streaming" => GrpcMethod::ClientStreaming,
            "bidirectional-streaming" | "bidi-streaming" => GrpcMethod::BidirectionalStreaming,
            _ => GrpcMethod::Custom(s),
        }
    }
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
pub struct Request {
    pub id: i32,
    #[validate(email)]
    pub user_email: Option<String>,
    #[validate(length(min = 1))]
    pub url: String,
    pub method: GrpcMethod,
    pub metadata: Option<String>,
    pub payload: Option<String>,
    #[validate(length(min = 1))]
    pub status: String,
    pub service: Option<String>,
    pub proto_file: Option<String>,
    #[validate(length(min = 1))]
    pub date: String,
    pub hidden: bool,
}
