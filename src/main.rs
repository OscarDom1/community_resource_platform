mod config;
mod db;
mod models;
mod routes;
mod services;

use actix_web::{App, HttpServer, web};
use actix_web::middleware::Logger;
use log::info;
use dotenv::dotenv;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize the logger (typically done at the start of the program)
    env_logger::init();

    // Log an info-level message indicating the server has started
    info!("Starting Actix Web server...");

   // Load configuration from environment variables or other sources
   dotenv().ok();  // Ensure .env file is loaded
   let config = config::Config::from_env();  // Directly use from_env

   // Initialize the database connection pool
   let pool = db::init_pool(&config.database_url)
       .await
       .expect("Failed to connect to the database");

    // Log the successful database connection
    info!("Connected to the database successfully.");

    // Set up and run the Actix Web server
    HttpServer::new(move || {
        App::new()
            // Attach the database connection pool to the application
            .app_data(web::Data::new(pool.clone())) 
            // Use the Logger middleware to log incoming HTTP requests
            .wrap(Logger::default()) 
            // Configure routes from the routes module
            .configure(routes::init)
    })
    // Bind to the local address and port (could also use env variable here for flexibility)
    .bind(("127.0.0.1", 8082))? 
    .run()
    .await
}
