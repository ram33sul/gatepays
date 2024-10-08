use argon2::{password_hash::SaltString, Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use rand::rngs::OsRng;

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
