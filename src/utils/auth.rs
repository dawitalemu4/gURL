use std::time::{SystemTime, UNIX_EPOCH};

use bcrypt::{DEFAULT_COST, hash, verify};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use miette::{Result, miette};
use serde::{Deserialize, Serialize};

use crate::{models::user::User, utils::env::env};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: User, // has to be a string, serde this
    pub exp: u64, // expiration time
    pub iat: u64, // issued at
}

pub fn hash_password(password: &str) -> Result<String> {
    hash(password, DEFAULT_COST).map_err(|e| miette!("Failed to hash password: {e}"))
}

pub fn verify_password(password: &str, hashed_password: &str) -> Result<bool> {
    verify(password, hashed_password).map_err(|e| miette!("Failed to verify password: {e}"))
}

pub fn create_jwt(user: User) -> Result<String> {
    let env = env()?;
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|e| miette!("Failed to get current time: {e}"))?
        .as_secs();

    let claims = Claims {
        sub: user,
        exp: now + (24 * 60 * 60 * 504),
        iat: now,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(env.jwt_signature.as_ref()),
    )
    .map_err(|e| miette!("Failed to generate JWT: {e}"))
}

pub fn parse_jwt(token: &str) -> Result<User> {
    let env = env()?;
    let mut validation = Validation::default();
    validation.insecure_disable_signature_validation();
    validation.validate_exp = false;

    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(env.jwt_signature.as_ref()),
        &validation
    );

    eprintln!("{:#?}", token_data);

    Ok(token_data.map_err(|e| miette!("Invalid JWT token: {e}"))?.claims.sub)
}
