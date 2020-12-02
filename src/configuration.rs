use lazy_static::lazy_static;

lazy_static! {
    pub static ref CONFIG: Settings = {
        let mut settings = config::Config::default();
        let file = config::File::with_name("config");
        settings.merge(file).expect("failed to read config");
        settings
            .try_into()
            .expect("failed to convert config into settings")
    };
}

#[derive(serde::Deserialize, Clone)]
pub struct Settings {
    pub server_port: u16,
    pub server_host: String,
    pub database: DatabaseSettings,
}

#[derive(serde::Deserialize, Clone)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: String,
    pub port: u16,
    pub host: String,
    pub database_name: String,
}

impl DatabaseSettings {
    pub fn connection_string(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username, self.password, self.host, self.port, self.database_name
        )
    }

    pub fn connection_string_without_db(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}",
            self.username, self.password, self.host, self.port
        )
    }
}
