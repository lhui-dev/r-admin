use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct AuthUserProfile {
    pub id: i64,
    pub username: String,
    pub nickname: String,
    pub real_name: Option<String>,
    pub is_super_admin: bool,
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub access_token: String,
    pub token_type: &'static str,
    pub expires_in: u64,
    pub user: AuthUserProfile,
}

#[derive(Debug, Serialize)]
pub struct CurrentUserResponse {
    // This payload is the bootstrap shape for authenticated frontend state and
    // will later become the natural extension point for menu or tenant context.
    pub user: AuthUserProfile,
    pub roles: Vec<String>,
    pub permissions: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct LogoutResponse {
    pub logged_out: bool,
}
