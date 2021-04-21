#[macro_use]
extern crate diesel;
#[macro_use]
extern crate failure;
#[macro_use]
extern crate log;
#[macro_use]
extern crate actix_cors;

use env_logger;
use std::env;
use actix_web::{ HttpServer };

pub mod service;
pub mod model;
pub mod message;
pub mod message_handler;
pub mod schema;
pub mod db;
pub mod error;
pub mod prelude;
pub mod app;
pub mod controller;
pub mod route;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    if env::var("RUST_LOG").ok().is_none() {
        env::set_var("RUST_LOG", "info");
    }
    env_logger::init();

    let bind_address = env::var("BIND_ADDRESS").expect("BIND_ADDRESS is not set");

    HttpServer::new(||app::init())
        .bind(&bind_address)
        .unwrap_or_else(|_| panic!("Could not bind server to address {}", &bind_address))
        .run()
        .await
}