use std::net::TcpListener;

use cocktail_catalogue::{
    configuration::{AppSettings, ServerSettings},
    logging::get_subscriber_with_fallback,
};
use once_cell::sync::Lazy;
use sqlx::SqlitePool;
use tracing_subscriber::util::SubscriberInitExt;

use self::graphql_client::GraphQLClient;

mod graphql_client;

// If the a global tracing subscriber is set as default (via .init()) more than
// once, a panic will occur. We use once_cell to only initialise once.
static TRACING: Lazy<()> = Lazy::new(|| {
    let fallback_log_level = "cocktail_catalogue=debug,tower_http=debug";

    if std::env::var("TEST_LOG").is_ok() {
        get_subscriber_with_fallback(fallback_log_level, std::io::stdout).init();
    } else {
        get_subscriber_with_fallback(fallback_log_level, std::io::sink).init();
    };
});

pub struct TestApp {
    pub client: GraphQLClient,
}

pub fn spawn_app(db: SqlitePool) -> TestApp {
    Lazy::force(&TRACING);

    // Tests will use a random port on the host machine so each test gets its
    // own server. This avoids tests interfering with each other.
    let config = AppSettings {
        server: ServerSettings {
            host: "127.0.0.1".to_string(),
            port: 0,
        },
    };

    let listener = TcpListener::bind((config.server.host.as_ref(), config.server.port))
        .expect("Failed to bind random port");

    let url = format!("http://{}", listener.local_addr().unwrap());
    let client = GraphQLClient::new(url);

    tokio::spawn(async move {
        cocktail_catalogue::run(listener, db).await.unwrap();
    });

    TestApp { client }
}
