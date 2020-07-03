use std::sync::{Arc, Mutex};

mod database;
mod server;
mod schema;
mod utils;

const DB_LOCATION: &str = "test.db";

// FIXME: Don't return Result here.. Handle the error! Do something fancy like
// sending myself a message if everything crashes or simply printing the issue.
#[actix_rt::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db = Mutex::new(database::Database::open(DB_LOCATION)?);

    let sch = Arc::new(schema::create_schema());
    let ctx = Arc::new(schema::Context { db });

    server::start(sch, ctx)?.await?;

    Ok(())
}

