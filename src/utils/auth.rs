use bcrypt::{DEFAULT_COST, hash, verify};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use miette::{Result, miette};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::utils::env::env;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // email
    pub exp: u64,    // expiration time
    pub iat: u64,    // issued at
}

pub fn hash_password(password: &str) -> Result<String> {
    hash(password, DEFAULT_COST).map_err(|e| miette!("Failed to hash password: {e}"))
}

pub fn verify_password(password: &str, hash: &str) -> Result<bool> {
    verify(password, hash).map_err(|e| miette!("Failed to verify password: {e}"))
}

pub fn generate_jwt(email: &str) -> Result<String> {
    let env = env()?;
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|e| miette!("Failed to get current time: {e}"))?
        .as_secs();

    let claims = Claims {
        sub: email.to_string(),
        exp: now + (24 * 60 * 60), // 24 hours
        iat: now,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(env.jwt_signature.as_ref()),
    )
    .map_err(|e| miette!("Failed to generate JWT: {e}"))
}

pub fn validate_jwt(token: &str) -> Result<Claims> {
    let env = env()?;
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(env.jwt_signature.as_ref()),
        &Validation::default(),
    )
    .map_err(|e| miette!("Invalid JWT token: {e}"))?;

    Ok(token_data.claims)
}

pub fn extract_token_from_header(auth_header: &str) -> Result<String> {
    if auth_header.starts_with("Bearer ") {
        Ok(auth_header[7..].to_string())
    } else {
        Err(miette!("Invalid authorization header format"))
    }
}
