#[derive(Debug)]
pub struct AppSettings {
    pub server: ServerSettings,
}

#[derive(Debug)]
pub struct ServerSettings {
    pub port: u16,
    pub host: String,
}

impl ServerSettings {
    pub fn address(&self) -> String {
        format!("http://{}:{}", self.host, self.port)
    }
}
