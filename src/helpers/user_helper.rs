use std::time::{SystemTime, UNIX_EPOCH};

use argon2::{password_hash::SaltString, Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use jsonwebtoken::{self, decode, encode, DecodingKey, EncodingKey, Header, Validation};
use rand::rngs::OsRng;
use serde::{Deserialize, Serialize};

use crate::config;

pub fn hash_password(password: &String) -> Result<String, argon2::password_hash::Error> {
    let salt = SaltString::generate(&mut OsRng);
    let hash = Argon2::default()
        .hash_password(password.as_bytes(), &salt)?
        .to_string();
    Ok(hash)
}

pub fn verify_password(
    hash: &String,
    password: &String,
) -> Result<bool, argon2::password_hash::Error> {
    let parsed_hash = PasswordHash::new(hash)?;
    Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .map(|_| true)
        .or_else(|_| Ok(false))
}

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub user_id: i32,
    pub exp: u64,
}

pub fn sign_jwt(user_id: &i32) -> Result<String, jsonwebtoken::errors::Error> {
    let expiration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backward")
        .as_secs()
        + 60 * 60 * 24 * 30 * 12;
    let claims = Claims {
        user_id: user_id.to_owned(),
        exp: expiration,
    };
    let env_config = config::Config::from_env();
    let token = encode(
        &Header::new(jsonwebtoken::Algorithm::HS256),
        &claims,
        &EncodingKey::from_secret(env_config.jwt_secret.as_ref()),
    )?;
    Ok(token)
}

pub fn verify_jwt(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let validation = Validation::new(jsonwebtoken::Algorithm::HS256);
    let env_config = config::Config::from_env();
    let token_data = decode(
        &token,
        &DecodingKey::from_secret(env_config.jwt_secret.as_ref()),
        &validation,
    )?;
    Ok(token_data.claims)
}
