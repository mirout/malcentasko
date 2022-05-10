use std::error::Error;

use actix_web::{web, App, HttpServer};

#[macro_use]
extern crate diesel_migrations;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

mod config;
mod errors;
mod handlers;
mod macros;
mod models;
mod schema;
mod services;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().expect("Failed to read .env file");

    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL not found");
    let port = std::env::var("MALCENTASKO_PORT")
        .map_err(|err| Box::<dyn Error>::from(err))
        .and_then(|f| f.parse().map_err(|err| Box::<dyn Error>::from(err)))
        .unwrap_or(8080);
    let host = std::env::var("MALCENTASKO_HOST").unwrap_or("127.0.0.1".to_string());

    let pool = config::get_pool(&db_url);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .configure(config::config_from_env)
    })
    .bind((host, port))?
    .run()
    .await
}
