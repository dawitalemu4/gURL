use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use validator::Validate;

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
pub struct Request {
    pub id: Option<i32>,
    #[validate(email)]
    pub user_email: Option<String>,
    #[validate(length(min = 1))]
    pub command: String,
    pub status: Option<String>,
    pub method: Option<String>,
    #[validate(length(min = 1))]
    pub date: String,
    pub hidden: bool,
}
