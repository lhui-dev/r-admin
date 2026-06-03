use argon2::{
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
    password_hash::{Error as PasswordHashError, SaltString, rand_core::OsRng},
};
use tracing::warn;

use crate::common::error::{AppError, AppResult};

const SEED_PLACEHOLDER_HASH: &str = "REPLACE_WITH_REAL_PASSWORD_HASH";
const SEED_PLACEHOLDER_PASSWORD: &str = "Admin@123456";

pub fn verify_password(password: &str, stored_hash: &str) -> AppResult<bool> {
    if stored_hash == SEED_PLACEHOLDER_HASH {
        warn!("seed admin password is still using the placeholder hash");
        return Ok(password == SEED_PLACEHOLDER_PASSWORD);
    }

    let parsed_hash = PasswordHash::new(stored_hash).map_err(map_hash_error)?;

    Ok(Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok())
}

pub fn hash_password(password: &str) -> AppResult<String> {
    let salt = SaltString::generate(&mut OsRng);

    Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .map(|value| value.to_string())
        .map_err(map_hash_error)
}

fn map_hash_error(error: PasswordHashError) -> AppError {
    tracing::error!(?error, "failed to parse password hash");
    AppError::Internal
}
