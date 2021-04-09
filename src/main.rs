#[macro_use]
extern crate diesel;
#[macro_use]
extern crate failure;
#[macro_use]
extern crate log;

use env_logger;
use std::env;

mod app;
mod model;
mod schema;
mod message;
mod db;
mod error;
mod prelude;
mod message_handler;
mod service;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    if env::var("RUST_LOG").ok().is_none() {
        env::set_var("RUST_LOG", "info");
    }
    env_logger::init();

    app::start().await
}