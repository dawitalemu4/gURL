use std::env;

use miette::{Result, miette};
use validator::Validate;

#[derive(Debug, Clone, Validate)]
pub struct Env {
    pub db_name: String,
    pub port: String,
    #[validate(length(min = 1))]
    pub jwt_signature: String,
}

pub fn env() -> Result<Env> {
    match dotenv::dotenv().is_ok() {
        false => Err(miette!("dotenv can't find .env file")),
        _ => {
            let db_name = env::var("DB_NAME").unwrap_or_default();
            let port = env::var("PORT").unwrap_or(9000.to_string());
            let jwt_signature = env::var("JWT_SIGNATURE").unwrap_or("blah".to_string());

            Ok(Env {
                db_name,
                port,
                jwt_signature,
            })
        }
    }
}
