use cocktail_catalogue_backend::server;
use std::net::TcpListener;

// FIXME: Don't return Result here.. Handle the error! Do something fancy like
// sending myself a message if everything crashes or simply printing the issue.
#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind(server::HOST)?;
    server::start(listener)?.await
}
