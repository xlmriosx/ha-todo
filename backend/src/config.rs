#[derive(Clone)]
pub struct Config {
    pub database_url: String,
    pub server_host: String,
    pub server_port: u16,
}

impl Config {
    pub fn from_env() -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Config {
            database_url: std::env::var("DATABASE_URL")
                .map_err(|_| "DATABASE_URL must be set in .env file")?,
            server_host: std::env::var("SERVER_HOST")
                .unwrap_or_else(|_| "localhost".to_string()),
            server_port: std::env::var("SERVER_PORT")
                .unwrap_or_else(|_| "8080".to_string())
                .parse()
                .map_err(|_| "SERVER_PORT must be a valid number")?,
        })
    }
}