use std::time::{SystemTime, UNIX_EPOCH};

use argon2::{
    Argon2,
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use miette::{Result, miette};
use serde::{Deserialize, Serialize};

use crate::{models::user::User, utils::env::env};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: u64, // expiration time
    pub iat: u64, // issued at
}

pub fn hash_password(password: &str) -> Result<String> {
    // In real applications you should never use a constant salt!!
    // Checkout https://cheatsheetseries.owasp.org/cheatsheets/Password_Storage_Cheat_Sheet.html#salting for details
    let salt = SaltString::from_b64("dec7901d02ee422ba6bd0333e4fef137")
        .map_err(|e| miette!("Failed to create salt: {e}"))?;
    Ok(Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .map_err(|e| miette!("Failed to hash password: {e}"))?
        .to_string())
}

pub fn verify_password(password: &str, hashed_password: &str) -> Result<bool> {
    let parsed_hash = PasswordHash::new(&hashed_password)
        .map_err(|e| miette!("Failed to parse hashed password: {e}"))?;
    Ok(Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok())
}

pub fn create_jwt(user: User) -> Result<String> {
    let env = env()?;
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|e| miette!("Failed to get current time: {e}"))?
        .as_secs();

    let claims = Claims {
        sub: serde_json::to_string(&user)
            .map_err(|e| miette!("Failed to serialize User for JWT: {e}"))?,
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

    // Don't care about these because it's a self hosted app
    validation.insecure_disable_signature_validation();
    validation.validate_exp = false;

    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(env.jwt_signature.as_ref()),
        &validation,
    )
    .map_err(|e| miette!("Invalid JWT token: {e}"))?;

    let parsed_token = serde_json::from_str(&token_data.claims.sub)
        .map_err(|e| miette!("Failed to deserialize User from JWT: {e}"))?;

    Ok(parsed_token)
}
