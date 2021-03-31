use env_logger;
use std::env;
use actix;

pub mod app;
pub mod model;
pub mod schema;
pub mod message;
pub mod db;
mod error;

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