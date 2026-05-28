mod app;
mod common;
mod config;
mod middleware;
mod modules;
mod routes;
mod state;

use anyhow::Context;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    let settings = config::Settings::load().context("failed to load application settings")?;
    app::bootstrap_tracing(&settings.log.level);

    let pool = state::build_db_pool(&settings.database.url)
        .await
        .context("failed to initialize database pool")?;

    app::run(settings, pool).await
}
