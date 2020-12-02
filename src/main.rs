use cocktail_catalogue_backend::{configuration::CONFIG, server};
use sqlx::PgPool;
use std::net::TcpListener;

// FIXME: Don't return Result here.. Handle the error! Do something fancy like
// sending myself a message if everything crashes or simply printing the issue.
#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let address = format!("{}:{}", CONFIG.server_host, CONFIG.server_port);
    let listener = TcpListener::bind(&address)?;
    let db_pool = PgPool::connect(&CONFIG.database.connection_string())
        .await
        .expect("failed to connect to postgres");

    server::start(listener, db_pool)?.await
}
