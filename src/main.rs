
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

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    if env::var("RUST_LOG").ok().is_none() {
        env::set_var("RUST_LOG", "info");
    }
    env_logger::init();

    app::start().await
}