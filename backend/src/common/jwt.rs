use std::time::{SystemTime, UNIX_EPOCH};

use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};
use tracing::error;

use crate::common::error::{AppError, AppResult};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessTokenClaims {
    // Claims are intentionally kept small for the current phase. The backend
    // still re-checks user status and permissions from the database.
    pub sub: i64,
    pub username: String,
    pub is_super_admin: bool,
    pub iat: u64,
    pub exp: u64,
}

pub fn generate_access_token(
    user_id: i64,
    username: &str,
    is_super_admin: bool,
    secret: &str,
    expires_in: u64,
) -> AppResult<String> {
    let issued_at = current_unix_timestamp()?;
    let claims = AccessTokenClaims {
        sub: user_id,
        username: username.to_string(),
        is_super_admin,
        iat: issued_at,
        exp: issued_at.saturating_add(expires_in),
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .map_err(|error| {
        error!(?error, "failed to generate jwt access token");
        AppError::Internal
    })
}

pub fn decode_access_token(token: &str, secret: &str) -> AppResult<AccessTokenClaims> {
    decode::<AccessTokenClaims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )
    .map(|data| data.claims)
    .map_err(|error| {
        error!(?error, "failed to decode jwt access token");
        AppError::unauthorized("invalid or expired access token")
    })
}

fn current_unix_timestamp() -> AppResult<u64> {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_secs())
        .map_err(|error| {
            error!(?error, "failed to calculate current unix timestamp");
            AppError::Internal
        })
}
