use sqlx::{postgres::PgPoolOptions, PgPool};

use crate::config::AppSettings;

#[derive(Clone)]
pub struct AppState {
    app: AppSettings,
    pub db: PgPool,
}

impl AppState {
    pub fn new(app: AppSettings, db: PgPool) -> Self {
        Self { app, db }
    }

    pub fn app_name(&self) -> &str {
        &self.app.name
    }
}

pub async fn build_db_pool(database_url: &str) -> Result<PgPool, sqlx::Error> {
    PgPoolOptions::new()
        .max_connections(10)
        .connect(database_url)
        .await
}
