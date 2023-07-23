use cocktail_catalogue::{
    configuration::{AppSettings, DatabaseSettings, ServerSettings},
    logging,
};
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
        database: DatabaseSettings::from_env()?,
    };

    cocktail_catalogue::run(&config).await?;

    Ok(())
}

