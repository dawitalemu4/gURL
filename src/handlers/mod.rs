use std::sync::{Arc, Mutex};

use axum::extract::State;
use miette::{Result, miette};
use rusqlite::{Connection, Statement, params_from_iter};
use serde::{Deserialize, Serialize};
use serde_with::{NoneAsEmptyString, serde_as, skip_serializing_none};
use validator::Validate;

pub mod grpcurl;
pub mod request;
pub mod template;
pub mod user;

pub use grpcurl::*;
pub use request::*;
pub use template::*;
pub use user::*;

use crate::models::{request::Request, user::User};

pub type ConnectionState = State<Arc<Mutex<Connection>>>;

#[serde_as]
#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PathParams {
    // #[validate(email)]
    #[serde_as(as = "NoneAsEmptyString")]
    email: Option<String>,
    #[serde(rename = "reqID")]
    // #[validate(length(min = 1))]
    #[serde_as(as = "NoneAsEmptyString")]
    request_id: Option<String>,
    // #[validate(length(min = 1))]
    // #[serde_as(as = "NoneAsEmptyString")]
    token: Option<String>,
    // #[validate(length(min = 1))]
    #[serde_as(as = "NoneAsEmptyString")]
    page: Option<String>,
    deleted: Option<bool>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
pub struct RequestBody {
    #[validate(nested)]
    request: Option<Request>,
    #[validate(nested)]
    user: Option<User>,
}

pub fn map_requests(mut statement: Statement<'_>, args: &[String]) -> Result<Vec<Request>> {
    let parsed_rows = statement
        .query_map(params_from_iter(args), |row| {
            Ok(Request {
                id: row.get(0)?,
                user_email: row.get::<_, Option<String>>(1)?,
                url: row.get(2)?,
                method: row.get::<_, String>(3)?.into(),
                origin: row.get::<_, Option<String>>(4)?,
                headers: row.get::<_, Option<String>>(5)?,
                body: row.get::<_, Option<String>>(6)?,
                status: row.get(7)?,
                date: row.get(8)?,
                hidden: row.get(9)?,
            })
        })
        .map_err(|e| miette!("Error mapping rows to Request: {e}"))?
        .map(|item| item.expect("Cannot unwrap Request row item"))
        .collect::<Vec<_>>();

    Ok(parsed_rows)
}

pub fn map_single_value(mut statement: Statement<'_>, args: &[String], value: &str) -> Result<Vec<String>> {
    let parsed_rows = statement
        .query_map(params_from_iter(args), |row| Ok(row.get(0)?))
        .map_err(|e| miette!("Error mapping {value}: {e}"))?
        .map(|item| item.expect("Cannot unwrap {value} row item"))
        .collect::<Vec<_>>();

    Ok(parsed_rows)
}

pub fn map_user(mut statement: Statement<'_>, args: &[String]) -> Result<Vec<User>> {
    let parsed_rows = statement
        .query_map(params_from_iter(args), |row| {
            Ok(User {
                username: row.get(0)?,
                email: row.get(1)?,
                password: row.get(2)?,
                favorites: row.get::<_, Option<Vec<i32>>>(3)?,
                date: row.get::<_, Option<String>>(4)?,
                old_pw: row.get(5)?,
                deleted: row.get(9)?,
            })
        })
        .map_err(|e| miette!("Error mapping rows to User: {e}"))?
        .map(|item| item.expect("Cannot unwrap User row item"))
        .collect::<Vec<_>>();

    Ok(parsed_rows)
}
