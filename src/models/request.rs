use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use validator::Validate;

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
pub struct Request {
    pub id: i32,
    #[validate(email)]
    pub user_email: Option<String>,
    #[validate(length(min = 1))]
    pub url: String,
    #[validate(length(min = 1))]
    pub method: String,
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
