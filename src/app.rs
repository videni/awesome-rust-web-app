use crate::db::{new_pool, ConnectionPool};
use crate::route;
use actix_web::{
    middleware::Logger,
    App, 
    Error,
    http::header::{AUTHORIZATION, CONTENT_TYPE},
};
use actix_cors::Cors;
use std::env;
use actix_web::dev::{Body, ServiceRequest, ServiceResponse};
use actix_service::ServiceFactory;

pub struct AppState {
    pub db_connection_pool: ConnectionPool 
}

pub enum AppEnv {
    Dev,
    Prod,
    Test
}

// Set up dotenv and logger
pub fn setup(env: AppEnv) {
    let mut env_filename = ".env";
    if let AppEnv::Test = env {
        env_filename= ".env.test"
    }

    dotenv::from_filename(env_filename).ok();

    dotenv::dotenv().ok();

    if env::var("RUST_LOG").ok().is_none() {
        env::set_var("RUST_LOG", "info");
    }
    env_logger::init();
}

pub fn boot() -> App<
    impl ServiceFactory<
        Request = ServiceRequest,
        Config = (),
        Response = ServiceResponse,
        Error = Error,
        InitError = (),
    >,
    Body,
> {
    let frontend_origin = env::var("FRONTEND_ORIGIN").ok();
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
        // .wrap(Logger::default())
        // .wrap(cors)
        .configure(route::routes)
}

fn create_state() -> AppState
{
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    println!("{}", &database_url);
    let database_pool = new_pool(database_url).expect("Failed to create pool.");

    AppState {
        db_connection_pool: database_pool
    }
}