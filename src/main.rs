use std::{env, net::TcpListener};

use cocktail_catalogue::{
    configuration::{AppSettings, ServerSettings},
    logging,
};
use eyre::Context;
use sqlx::SqlitePool;
use tracing_subscriber::util::SubscriberInitExt;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    color_eyre::install()?;

    logging::get_subscriber_with_fallback(
        "cocktail_catalogue=debug,tower_http=info",
        std::io::stdout,
    )
    .init();

    let config = AppSettings {
        server: ServerSettings {
            port: 1337,
            host: "127.0.0.1".to_string(),
        },
    };

    let db_connection_string = env::var("DATABASE_URL").wrap_err("DATABASE_URL must be set")?;

    let db = SqlitePool::connect(&db_connection_string)
        .await
        .wrap_err("Failed to connect to database")?;

    let listener = TcpListener::bind((config.server.host.as_ref(), config.server.port))
        .wrap_err_with(|| {
            format!(
                "Failed to bind address {}:{}",
                config.server.host, config.server.port
            )
        })?;

    cocktail_catalogue::run(listener, db).await?;

    Ok(())
}

