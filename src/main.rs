#[macro_use]
extern crate diesel;
#[macro_use]
extern crate thiserror;
#[macro_use]
extern crate log;
// #[macro_use]
// extern crate actix_cors;
#[macro_use]
extern crate lazy_static;

use actix_web::HttpServer;
use std::env;

pub mod app;
pub mod controller;
pub mod db;
pub mod error;
pub mod message;
pub mod message_handler;
pub mod middleware;
pub mod model;
pub mod prelude;
pub mod route;
pub mod schema;
pub mod service;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    app::setup(app::AppEnv::Prod);

    let bind_address = env::var("BIND_ADDRESS").expect("BIND_ADDRESS is not set");

    HttpServer::new(|| app::create())
        .bind(&bind_address)
        .unwrap_or_else(|_| panic!("Could not bind server to address {}", &bind_address))
        .run()
        .await
}
