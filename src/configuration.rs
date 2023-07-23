use std::env;

use eyre::Context;

#[derive(Debug)]
pub struct AppSettings {
    pub server: ServerSettings,
    pub database: DatabaseSettings,
}

#[derive(Debug)]
pub struct ServerSettings {
    pub port: u16,
    pub host: String,
}

#[derive(Debug)]
pub struct DatabaseSettings {
    pub connection_string: String,
}

impl DatabaseSettings {
    pub fn from_env() -> eyre::Result<Self> {
        Ok(Self {
            connection_string: env::var("DATABASE_URL").wrap_err("DATABASE_URL must be set")?,
        })
    }
}
