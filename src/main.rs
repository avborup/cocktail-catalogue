mod database;
mod server;
mod utils;

// FIXME: Don't return Result here..
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db = database::Database::open("test.db")?;
    dbg!{db.get_all_cocktails()?};

    // server::start("localhost:8000");

    Ok(())
}
