use crate::db::new_pool;
use crate::message_handler::MessageHandler;
use actix::prelude::{Addr, SyncArbiter};
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
    pub message_handler: Addr<MessageHandler>,
}

pub fn init() -> App<
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

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let database_pool = new_pool(database_url).expect("Failed to create pool.");
    
    let message_handler = SyncArbiter::start(num_cpus::get(), move || MessageHandler {
        db_connection_pool: database_pool.clone()
    });

    let state = AppState {
        message_handler: message_handler.clone(),
    };
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