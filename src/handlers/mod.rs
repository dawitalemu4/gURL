use std::sync::{Arc, Mutex};
use std::time::Duration;

use axum::extract::{Path, State};
use humantime::format_duration;
use miette::{Result, miette};
use rusqlite::{Connection, Statement, params_from_iter};
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, skip_serializing_none};
use validator::Validate;

pub mod grpcurl;
pub mod request;
pub mod template;
pub mod user;

pub use grpcurl::*;
pub use request::*;
pub use template::*;
pub use user::*;

use crate::models::{request::Request, user::User, deserialize_favorites_for_db, deserialize_bool_for_db};

pub type ConnectionState = State<Arc<Mutex<Connection>>>;

#[serde_as]
#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
pub struct PathParams {
    #[validate(email)]
    email: Option<String>,
    #[validate(length(min = 1))]
    password: Option<String>,
    #[serde(rename = "reqID")]
    #[validate(length(min = 1))]
    request_id: Option<String>,
    #[validate(length(min = 1))]
    token: Option<String>,
    #[validate(length(min = 1))]
    page: Option<String>,
    deleted: Option<bool>,
}

// Request/User utils
pub async fn get_all_requests_from_db(
    state: ConnectionState,
    Path(path): Path<PathParams>,
) -> Result<Vec<Request>> {
    let email = path.email.unwrap_or("anon".to_string());
    let db = state
        .lock()
        .map_err(|e| miette!("Global db can't block current thread {e}"))?;

    match db
        .prepare("SELECT * FROM request WHERE user_email = ?1 AND hidden = false ORDER BY id DESC")
    {
        Ok(rows) => {
            let requests = map_requests(rows, &[email])?;

            Ok(requests)
        }
        Err(e) => Err(miette!("{e}")),
    }
}

pub async fn get_all_favorites_from_db(
    state: ConnectionState,
    Path(path): Path<PathParams>,
) -> Result<Vec<Request>> {
    let mut favorite_requests = Vec::new();
    let email = path.email.unwrap_or("anon".to_string());
    let db = state
        .lock()
        .map_err(|e| miette!("Global db can't block current thread {e}"))?;

    let favorite_rows = match db.prepare(r#"SELECT favorites FROM "user" WHERE email = ?1"#) {
        Ok(rows) => rows,
        Err(e) => {
            return Err(miette!("{e}"));
        }
    };

    let favorite_ids = map_single_value(favorite_rows, &[email.clone()], "favorite")?;
    for favorite in favorite_ids {
        let rows = match db.prepare(
            r#"
                SELECT * FROM request WHERE user_email = ?1 AND id = ?2 AND hidden = false 
                ORDER BY id DESC
            "#,
        ) {
            Ok(rows) => rows,
            Err(e) => {
                return Err(miette!("{e}"));
            }
        };

        let res = map_requests(rows, &[email.clone(), favorite])?;
        favorite_requests.push(res[0].clone());
    }

    Ok(favorite_requests)
}

pub fn map_requests(mut statement: Statement<'_>, args: &[String]) -> Result<Vec<Request>> {
    let parsed_rows = statement
        .query_map(params_from_iter(args), |row| {
            Ok(Request {
                id: row.get(0)?,
                user_email: row.get::<_, Option<String>>(1)?,
                command: row.get(2)?,
                status: row.get(3)?,
                method: row.get(4)?,
                date: row.get(5)?,
                hidden: deserialize_bool_for_db(row.get(6)?),
            })
        })
        .map_err(|e| miette!("Error mapping rows to Request: {e}"))?
        .map(|item| item.expect("Cannot unwrap Request row item"))
        .collect::<Vec<_>>();

    Ok(parsed_rows)
}

pub fn map_single_value(
    mut statement: Statement<'_>,
    args: &[String],
    value: &str,
) -> Result<Vec<String>> {
    let parsed_rows = statement
        .query_map(params_from_iter(args), |row| Ok(row.get(0)?))
        .map_err(|e| miette!("_Error mapping {value}: {e}"))?
        .map(|item| item.expect("Cannot unwrap {value} row item"))
        .collect::<Vec<_>>();

    Ok(parsed_rows)
}

pub fn map_user(mut statement: Statement<'_>, args: &[String]) -> Result<User> {
    let parsed_rows = statement
        .query_map(params_from_iter(args), |row| {
            Ok(User {
                username: row.get(0)?,
                email: row.get(1)?,
                password: row.get(2)?,
                favorites: deserialize_favorites_for_db(row.get(3)?),
                date: row.get::<_, Option<String>>(4)?,
                deleted: deserialize_bool_for_db(row.get(5)?),
                old_pw: "".to_string()
            })
        })
        .map_err(|e| miette!("Error mapping rows to User: {e}"))?
        .map(|item| item.expect("Cannot unwrap User row item"))
        .collect::<Vec<_>>();

    if parsed_rows.len() > 0 {
        Ok(parsed_rows[0].clone())
    } else {
        Err(miette!("User error"))
    }
}

// Template utils
pub fn humanize_date(date: Option<String>) -> Result<String> {
    let date: u64 = date
        .unwrap_or(0.to_string())
        .parse()
        .map_err(|e| miette!("Could not parse date: {e}"))?;

    Ok(format_duration(Duration::from_millis(date)).to_string())
}

pub fn get_status_color(status: &Option<String>) -> String {
    let status_colors = std::collections::HashMap::from([
        // Success
        ("0", "green"), // OK
        // Client Error
        ("1", "red"),  // CANCELLED
        ("2", "red"),  // UNKNOWN
        ("3", "red"),  // INVALID_ARGUMENT
        ("4", "red"),  // DEADLINE_EXCEEDED
        ("5", "red"),  // NOT_FOUND
        ("6", "red"),  // ALREADY_EXISTS
        ("7", "red"),  // PERMISSION_DENIED
        ("8", "red"),  // RESOURCE_EXHAUSTED
        ("9", "red"),  // FAILED_PRECONDITION
        ("10", "red"), // ABORTED
        ("11", "red"), // OUT_OF_RANGE
        ("12", "red"), // UNIMPLEMENTED
        // Server Error
        ("13", "orange"), // INTERNAL
        ("14", "orange"), // UNAVAILABLE
        ("15", "orange"), // DATA_LOSS
        ("16", "red"),    // UNAUTHENTICATED
        // Custom
        ("other", "yellow"),
    ]);

    status_colors
        .get(status.clone().unwrap_or("other".to_string()).as_str())
        .map_or("other", |status| status)
        .to_string()
}
