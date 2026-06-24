/// Configuration struct for the application.
pub struct Config {
    pub database_url: String,
    pub server_port: u16,
}

impl Config {
    /// Loads configuration from environment variables.
    pub fn from_env() -> Result<Self, Box<dyn std::error::Error>> {
        let database_url = std::env::var("DATABASE_URL")?;
        let server_port = std::env::var("SERVER_PORT")?.parse::<u16>()?;
        Ok(Config { database_url, server_port })
    }
}