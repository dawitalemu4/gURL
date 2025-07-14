use std::sync::{Arc, Mutex};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

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

use crate::models::{
    deserialize_bool_for_db, deserialize_favorites_for_db, request::Request, user::User,
};

pub type ConnectionState = State<Arc<Mutex<Connection>>>;

#[serde_as]
#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
pub struct RequestBody {
    #[validate(length(min = 1))]
    command: String,
}

#[serde_as]
#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
pub struct PathParams {
    #[validate(length(min = 1))]
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

    Ok(map_requests(
        db.prepare(
            "SELECT * FROM request WHERE user_email = ?1 AND hidden = false ORDER BY id DESC",
        )
        .map_err(|e| miette!("Invalid statement: {e}"))?,
        &[email],
    )?)
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

    let favorite_ids = map_single_value(
        db.prepare(r#"SELECT favorites FROM "user" WHERE email = ?1"#)
            .map_err(|e| miette!("Invalid statement: {e}"))?,
        &[email.clone()],
        "favorite",
    )?;

    for favorite in favorite_ids {
        let res = map_requests(
            db.prepare(
                r#"
                SELECT * FROM request WHERE user_email = ?1 AND id = ?2 AND hidden = false 
                ORDER BY id DESC
            "#,
            )
            .map_err(|e| miette!("Invalid statement: {e}"))?,
            &[email.clone(), favorite],
        )?;

        if let Some(favorite_request) = res.first() {
            favorite_requests.push(favorite_request.clone());
        }
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

pub fn map_user(mut statement: Statement<'_>, args: &[&String]) -> Result<Vec<User>> {
    let parsed_rows = statement
        .query_map(params_from_iter(args), |row| {
            Ok(User {
                username: row.get(0)?,
                email: row.get(1)?,
                password: row.get(2)?,
                favorites: deserialize_favorites_for_db(row.get(3)?),
                date: row.get::<_, Option<String>>(4)?,
                deleted: deserialize_bool_for_db(row.get(5)?),
            })
        })
        .map_err(|e| miette!("Error mapping rows to User: {e}"))?
        .map(|item| item.expect("Cannot unwrap User row item"))
        .collect::<Vec<_>>();

    Ok(parsed_rows)
}

// Template utils
pub fn humanize_date(date: Option<String>) -> Result<String> {
    let date = if let Some(date) = date {
        let timestamp = date
            .parse::<u64>()
            .map_err(|e| miette!("Could not parse date to integer: {e}"))?;
        let target_time = UNIX_EPOCH + Duration::from_millis(timestamp);
        SystemTime::now()
            .duration_since(target_time)
            .map_err(|e| miette!("Could not get duration since provided date: {e}"))?
    } else {
        Duration::from_secs(0)
    };

    let human_date = format_duration(date)
        .to_string()
        .split_whitespace()
        .take(3)
        .collect::<Vec<_>>()
        .join(" ");

    Ok(human_date)
}

pub fn get_status_color(status: &Option<String>) -> String {
    let status_colors = std::collections::HashMap::from([
        // Success
        ("0", "green"), // OK
        ("OK", "green"),
        // Client Error
        ("1", "red"), // CANCELLED
        ("CANCELLED", "red"),
        ("2", "red"), // UNKNOWN
        ("UNKNOWN", "red"),
        ("3", "red"), // INVALID_ARGUMENT
        ("INVALID_ARGUMENT", "red"),
        ("4", "red"), // DEADLINE_EXCEEDED
        ("DEADLINE_EXCEEDED", "red"),
        ("5", "red"), // NOT_FOUND
        ("NOT_FOUND", "red"),
        ("6", "red"), // ALREADY_EXISTS
        ("ALREADY_EXISTS", "red"),
        ("7", "red"), // PERMISSION_DENIED
        ("PERMISSION_DENIED", "red"),
        ("8", "red"), // RESOURCE_EXHAUSTED
        ("RESOURCE_EXHAUSTED", "red"),
        ("9", "red"), // FAILED_PRECONDITION
        ("FAILED_PRECONDITION", "red"),
        ("10", "red"), // ABORTED
        ("ABORTED", "red"),
        ("11", "red"), // OUT_OF_RANGE
        ("OUT_OF_RANGE", "red"),
        ("12", "red"), // UNIMPLEMENTED
        ("UNIMPLEMENTED", "red"),
        // Server Error
        ("13", "orange"), // INTERNAL
        ("INTERNAL", "orange"),
        ("14", "orange"), // UNAVAILABLE
        ("UNAVAILABLE", "orange"),
        ("15", "orange"), // DATA_LOSS
        ("DATA_LOSS", "orange"),
        ("16", "red"), // UNAUTHENTICATED
        ("UNAUTHENTICATED", "red"),
        // Custom
        ("other", "yellow"),
    ]);

    status_colors
        .get(status.clone().unwrap_or("other".to_string()).as_str())
        .map_or("yellow", |status| status)
        .to_string()
}

pub fn get_service_name(command: &String) -> String {
    // let full_service = command.split(" ").last().unwrap_or_default();
    // let address = {
    //     let parts = command.split(" ").collect::<Vec<&str>>();
    //     if parts.len() >= 2 {
    //         parts[parts.len() - 2]
    //     } else {
    //         command.split(":").last().unwrap_or_default()
    //     }
    // };

    // format!(
    //     "{address} {}",
    //     full_service
    //         .split("/")
    //         .next()
    //         .unwrap_or(full_service)
    //         .to_string()
    // )

    command
        .split_whitespace()
        .filter(|word| !word.starts_with('-'))
        .collect::<Vec<_>>()
        .join(" ")
}
