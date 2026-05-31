use std::{
    collections::HashMap,
    sync::Arc,
    time::{SystemTime, UNIX_EPOCH},
};

use sqlx::{PgPool, postgres::PgPoolOptions};
use tokio::sync::RwLock;

use crate::config::{AppSettings, JwtSettings};

#[derive(Clone)]
pub struct AppState {
    app: AppSettings,
    pub jwt: JwtSettings,
    pub db: PgPool,
    // First-phase logout revocation store. This is intentionally in-memory for
    // single-instance development and should be replaced by a shared store later.
    revoked_tokens: Arc<RwLock<HashMap<String, u64>>>,
}

impl AppState {
    pub fn new(app: AppSettings, jwt: JwtSettings, db: PgPool) -> Self {
        Self {
            app,
            jwt,
            db,
            revoked_tokens: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn app_name(&self) -> &str {
        &self.app.name
    }

    pub async fn revoke_token(&self, token: String, expires_at: u64) {
        let mut revoked_tokens = self.revoked_tokens.write().await;
        revoked_tokens.insert(token, expires_at);
        prune_revoked_tokens(&mut revoked_tokens);
    }

    pub async fn is_token_revoked(&self, token: &str) -> bool {
        let mut revoked_tokens = self.revoked_tokens.write().await;
        prune_revoked_tokens(&mut revoked_tokens);
        revoked_tokens.contains_key(token)
    }
}

pub async fn build_db_pool(database_url: &str) -> Result<PgPool, sqlx::Error> {
    PgPoolOptions::new()
        .max_connections(10)
        .connect(database_url)
        .await
}

fn prune_revoked_tokens(revoked_tokens: &mut HashMap<String, u64>) {
    // Keep the revocation map bounded by dropping entries that are already expired.
    let now = current_unix_timestamp();
    revoked_tokens.retain(|_, expires_at| *expires_at > now);
}

fn current_unix_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_secs())
        .unwrap_or_default()
}
