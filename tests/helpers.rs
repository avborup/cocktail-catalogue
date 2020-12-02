use cocktail_catalogue_backend::configuration::{DatabaseSettings, CONFIG};
use cocktail_catalogue_backend::server;
use sqlx::{Connection, Executor, PgConnection, PgPool};
use std::net::TcpListener;

pub struct TestApp {
    pub address: String,
    pub db_pool: PgPool,
}

pub async fn spawn_app() -> TestApp {
    let listener = TcpListener::bind("127.0.0.1:0").expect("failed to bind port");
    let port = listener.local_addr().unwrap().port();
    let address = format!("http://127.0.0.1:{}", port);

    let mut cfg = CONFIG.clone();

    // Choose random database name to avoid separate tests connecting to the same one
    cfg.database.database_name = uuid::Uuid::new_v4().to_string();

    let db_pool = configure_database(&cfg.database).await;

    let server = server::start(listener, db_pool.clone()).expect("failed to start server");
    let _ = tokio::spawn(server);

    TestApp { address, db_pool }
}

async fn configure_database(db_cfg: &DatabaseSettings) -> PgPool {
    let mut conn = PgConnection::connect(&db_cfg.connection_string_without_db())
        .await
        .expect("failed to connect to postgres");

    conn.execute(&*format!(r#"CREATE DATABASE "{}";"#, db_cfg.database_name))
        .await
        .expect("failed to create database");

    let db_pool = PgPool::connect(&db_cfg.connection_string())
        .await
        .expect("failed to connect to postgres database");

    sqlx::migrate!("./migrations")
        .run(&db_pool)
        .await
        .expect("failed to migrate database");

    db_pool
}
