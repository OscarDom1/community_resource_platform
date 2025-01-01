use std::env;
use dotenv::dotenv;

pub struct Config {
    pub database_url: String,
    pub jwt_secret: String, // Add JWT secret configuration
    pub server_port: u16,   // Optional: Add server port configuration
}

impl Config {
    pub fn from_env() -> Self {
        dotenv().ok();
        Self {
            database_url: env::var("DATABASE_URL").expect("DATABASE_URL must be set"),
            jwt_secret: env::var("JWT_SECRET").expect("JWT_SECRET must be set"), // Fetch JWT secret from environment
            server_port: env::var("SERVER_PORT") // Optional: Set a default port if not provided
                .unwrap_or_else(|_| "8080".to_string())
                .parse()
                .expect("SERVER_PORT must be a valid number"),
        }
    }
}
