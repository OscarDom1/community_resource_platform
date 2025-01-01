mod config;
mod db;
mod models;
mod routes;

use actix_web::{App, HttpServer, web};
use actix_web::middleware::{Logger, DefaultHeaders};
use log::info;
use dotenv::dotenv;
use actix_web::http::header;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize the logger for structured logging
    env_logger::init();

    // Log an info-level message indicating the server is starting
    info!("Starting Actix Web server...");

    // Load configuration from environment variables or .env file
    dotenv().ok();
    let config = config::Config::from_env(); // Load configuration

    // Initialize the database connection pool
    let pool = db::init_pool(&config.database_url)
        .await
        .expect("Failed to connect to the database");

    // Log successful database connection
    info!("Connected to the database successfully.");

    // Start the Actix Web server
    HttpServer::new(move || {
        App::new()
            // Attach the database connection pool to the application
            .app_data(web::Data::new(pool.clone()))
            // Share the JWT secret with the application
            .app_data(web::Data::new(config.jwt_secret.clone()))
            // Use the Logger middleware to log incoming HTTP requests
            .wrap(Logger::default())
            // Use DefaultHeaders to ensure CORS headers are included
            .wrap(DefaultHeaders::new().add((header::ACCESS_CONTROL_ALLOW_ORIGIN, "*".parse::<header::HeaderValue>().expect("Invalid header value"))))
            // Configure routes from the routes module
            .configure(routes::init)
    })
    // Bind to the specified address and port
    .bind(("127.0.0.1", config.server_port))? // Use port from config
    .run()
    .await
}
