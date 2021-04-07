#[macro_use]
extern crate diesel;
#[macro_use]
extern crate failure;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate validator_derive;

use env_logger;
use std::env;
use actix;
mod app;
mod model;
mod schema;
mod message;
mod db;
mod error;
mod prelude;
mod message_handler;
mod service;

fn main()  {
    dotenv::dotenv().ok();

    if env::var("RUST_LOG").ok().is_none() {
        env::set_var("RUST_LOG", "conduit=debug,actix_web=info");
    }
    env_logger::init();

    let system = actix::System::new();

    app::start();

    system.run();
}