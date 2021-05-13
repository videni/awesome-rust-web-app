use crate::db::{new_pool, ConnectionPool};
use crate::route;
use actix_web::middleware::{Compat, Logger};
use actix_web::{
    http::header::{AUTHORIZATION, CONTENT_TYPE},
    App, 
    Error,
};
// use actix_cors::Cors;
use crate::middleware::negotiate_language::{NegotiateLanguage};
use actix_service::ServiceFactory;
use actix_web::dev::{Body, ServiceRequest, ServiceResponse};
use std::env;

pub struct AppState {
    pub db_connection_pool: ConnectionPool,
}

pub enum AppEnv {
    Dev,
    Prod,
    Test,
}

// Set up dotenv and logger
pub fn setup(env: AppEnv) {
    let mut env_filename = ".env";
    if let AppEnv::Test = env {
        env_filename = ".env.test"
    }

    dotenv::from_filename(env_filename).ok();

    dotenv::dotenv().ok();

    if env::var("RUST_LOG").ok().is_none() {
        env::set_var("RUST_LOG", "info");
    }
    env_logger::init();
}

pub fn create() -> App<
    impl ServiceFactory<
        ServiceRequest,
        Config = (),
        Response = ServiceResponse,
        Error = Error,
        InitError = (),
    >, Body> {
    let _frontend_origin = env::var("FRONTEND_ORIGIN").ok();
    let state = create_state();

    // let cors = match frontend_origin {
    //     Some(ref origin) => Cors::default()
    //         .allowed_origin(origin)
    //         .allowed_headers(vec![AUTHORIZATION, CONTENT_TYPE])
    //         .max_age(3600),
    //     None => Cors::default()
    //         .allowed_origin("*")
    //         .send_wildcard()
    //         .allowed_headers(vec![AUTHORIZATION, CONTENT_TYPE])
    //         .max_age(3600),
    // };

    App::new()
        .data(state)
        .wrap(Compat::new(Logger::default()))
        .wrap(NegotiateLanguage::default())
        // .wrap(cors)
        .configure(route::routes)
}

fn create_state() -> AppState {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    println!("{}", &database_url);
    let database_pool = new_pool(database_url).expect("Failed to create pool.");

    AppState {
        db_connection_pool: database_pool,
    }
}
