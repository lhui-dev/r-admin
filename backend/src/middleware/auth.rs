use axum::{
    extract::FromRequestParts,
    http::{header, request::Parts},
};

use crate::{
    common::{
        error::{AppError, AppResult},
        jwt,
    },
    state::AppState,
};

#[derive(Debug, Clone)]
pub struct AuthUser {
    pub user_id: i64,
    // Keep request auth context minimal so later permission checks do not
    // accidentally trust stale business fields embedded in old tokens.
    pub access_token: String,
    pub access_token_expires_at: u64,
}

impl FromRequestParts<AppState> for AuthUser {
    type Rejection = AppError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let token = bearer_token(parts)?;
        let claims = jwt::decode_access_token(token, &state.jwt.secret)?;

        // Logout revocation is checked here so every protected route gets the
        // same behavior without repeating token blacklist logic in handlers.
        if state.is_token_revoked(token).await {
            return Err(AppError::unauthorized("access token has been revoked"));
        }

        Ok(Self {
            user_id: claims.sub,
            access_token: token.to_string(),
            access_token_expires_at: claims.exp,
        })
    }
}

fn bearer_token(parts: &Parts) -> AppResult<&str> {
    let authorization = parts
        .headers
        .get(header::AUTHORIZATION)
        .and_then(|value| value.to_str().ok())
        .ok_or_else(|| AppError::unauthorized("missing authorization header"))?;

    let (scheme, token) = authorization
        .split_once(' ')
        .ok_or_else(|| AppError::unauthorized("invalid authorization header"))?;

    if !scheme.eq_ignore_ascii_case("bearer") || token.trim().is_empty() {
        return Err(AppError::unauthorized("invalid authorization header"));
    }

    Ok(token.trim())
}
