use std::net::SocketAddr;

use anyhow::Context;
use sqlx::PgPool;
use tokio::net::TcpListener;
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

use crate::{config::Settings, routes, state::AppState};

pub fn bootstrap_tracing(default_level: &str) {
    let env_filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(default_level));

    tracing_subscriber::registry()
        .with(env_filter)
        .with(tracing_subscriber::fmt::layer())
        .init();
}

pub async fn run(settings: Settings, pool: PgPool) -> anyhow::Result<()> {
    let app_state = AppState::new(settings.app, pool);
    let router = routes::build_router(app_state)
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http());

    let addr = SocketAddr::from((settings.server.host, settings.server.port));
    let listener = TcpListener::bind(addr)
        .await
        .with_context(|| format!("failed to bind tcp listener on {addr}"))?;

    info!("backend listening on http://{addr}");

    axum::serve(listener, router)
        .await
        .context("axum server exited unexpectedly")
}
