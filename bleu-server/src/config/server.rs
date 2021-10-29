use std::env;

pub struct ServerConfig {
    host: String,
    port: String,
}

impl ServerConfig {
    pub fn load() -> Self {
        let host = env::var("SERVER_HOST").expect("SERVER_HOST does not exist!");
        let port = env::var("SERVER_PORT").expect("SERVER_PORT does not exist!");
        Self {
            host,
            port,
        }
    }

    pub fn get_binding_url(&self) -> String {
        format!("{host}:{port}", host = self.host, port = self.port)
    }
}