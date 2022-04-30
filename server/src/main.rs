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
mod models;
mod schema;
mod services;

fn main() {
    println!("Hello, world!");
}
