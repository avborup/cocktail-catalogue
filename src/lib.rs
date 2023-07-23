use std::net::TcpListener;

use axum::{http, response::IntoResponse, routing::get, Router};
use configuration::AppSettings;
use eyre::Context;
use sqlx::SqlitePool;

pub mod configuration;

pub async fn create_app(config: &AppSettings) -> eyre::Result<Router> {
    let db = SqlitePool::connect(&config.database.connection_string)
        .await
        .wrap_err("Failed to connect to database")?;

    let server_state = ServerState { _db: db };

    let router = Router::new()
        .route("/health_check", get(health_check))
        .with_state(server_state);

    Ok(router)
}

#[derive(Debug, Clone)]
pub struct ServerState {
    _db: SqlitePool,
}

pub async fn run(config: &AppSettings) -> eyre::Result<()> {
    let app = create_app(config).await?;

    let listener = TcpListener::bind((config.server.host.as_str(), config.server.port))?;

    axum::Server::from_tcp(listener)?
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

async fn health_check() -> impl IntoResponse {
    http::StatusCode::OK
}
