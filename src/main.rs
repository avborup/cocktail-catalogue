use cocktail_catalogue::configuration::{AppSettings, DatabaseSettings, ServerSettings};

#[tokio::main]
async fn main() -> eyre::Result<()> {
    color_eyre::install()?;

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

