use crate::db::new_pool;
use crate::message_handler::MessageHandler;
use actix::prelude::{Addr, SyncArbiter};
use crate::controller::user::{login, register};
use actix_web::{
    middleware::Logger,
    web,
    App, 
    HttpServer,
    http::header::{AUTHORIZATION, CONTENT_TYPE},
};
use actix_cors::Cors;
use std::env;

pub struct AppState {
    pub message_handler: Addr<MessageHandler>,
}

pub async fn start() -> std::io::Result<()> {
    let frontend_origin = env::var("FRONTEND_ORIGIN").ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let database_pool = new_pool(database_url).expect("Failed to create pool.");
    
    let message_handler = SyncArbiter::start(num_cpus::get(), move || MessageHandler {
        db_connection_pool: database_pool.clone()
    });

    let bind_address = env::var("BIND_ADDRESS").expect("BIND_ADDRESS is not set");

    HttpServer::new(move || {
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
            .wrap(Logger::default())
            // .wrap(cors)
            .configure(routes)
        })
        .bind(&bind_address)
        .unwrap_or_else(|_| panic!("Could not bind server to address {}", &bind_address))
        .run()
        .await
}

async fn index() -> &'static str {
    "Hello, there!"
}

fn routes(app: &mut web::ServiceConfig) {
    app
        .service(web::resource("/").route(web::post().to(index)))
        .service(web::scope("/api")
                .service(web::resource("login")
                    .route(web::post().to(login))
                )
                .service(web::resource("register")
                    .route(web::post().to(register))
                )
            );
}
