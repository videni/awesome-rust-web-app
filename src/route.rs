use crate::controller::*;
use actix_web::web;

async fn index() -> &'static str {
    "Hello, there!"
}

pub fn routes(app: &mut web::ServiceConfig) {
    app.service(web::resource("/").route(web::post().to(index)))
        .service(
            web::scope("/api")
                .service(web::resource("login").route(web::post().to(login_controller::login)))
                .service(
                    web::resource("register").route(web::post().to(register_controller::register)),
                ),
        );
}
