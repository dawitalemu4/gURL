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

use crate::models::{request::Request, user::User};

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

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
pub struct RequestBody {
    #[validate(nested)]
    request: Option<Request>,
    #[validate(nested)]
    user: Option<User>,
}

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
                url: row.get(2)?,
                method: row.get::<_, String>(3)?.into(),
                metadata: row.get::<_, Option<String>>(4)?,
                payload: row.get::<_, Option<String>>(5)?,
                status: row.get(6)?,
                date: row.get(7)?,
                service: row.get::<_, Option<String>>(8)?,
                proto_file: row.get::<_, Option<String>>(9)?,
                hidden: row.get(10)?,
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
        .map_err(|e| miette!("Error mapping {value}: {e}"))?
        .map(|item| item.expect("Cannot unwrap {value} row item"))
        .collect::<Vec<_>>();

    Ok(parsed_rows)
}

pub fn map_user(mut statement: Statement<'_>, args: &[String]) -> Result<User> {
    let parsed_rows = statement
        .query_map(params_from_iter(args), |row| {
            let favorites_str: Option<String> = row.get(3)?;
            let favorites = favorites_str.filter(|s| !s.is_empty()).map(|s| {
                s.split(',')
                    .filter_map(|n| n.trim().parse::<i32>().ok())
                    .collect::<Vec<i32>>()
            });

            Ok(User {
                username: row.get(0)?,
                email: row.get(1)?,
                password: row.get(2)?,
                favorites,
                date: row.get::<_, Option<String>>(4)?,
                old_pw: row.get(5)?,
                deleted: row.get(9)?,
            })
        })
        .map_err(|e| miette!("Error mapping rows to User: {e}"))?
        .map(|item| item.expect("Cannot unwrap User row item"))
        .collect::<Vec<_>>();

    Ok(parsed_rows[0].clone())
}

pub fn serialize_favorites_for_db(favorites: &Option<Vec<i32>>) -> String {
    match favorites {
        Some(favorites) => {
            let csv = favorites
                .iter()
                .map(|n| n.to_string())
                .collect::<Vec<_>>()
                .join(",");
            csv
        }
        None => String::new(),
    }
}

pub fn humanize_date(date: Option<String>) -> Result<String> {
    let date: u64 = date
        .unwrap_or(0.to_string())
        .parse()
        .map_err(|e| miette!("Could not parse date: {e}"))?;

    Ok(format_duration(Duration::from_millis(date)).to_string())
}
