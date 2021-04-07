use actix_web::{HttpResponse, web, Error};
use crate::message::user::Login;
use crate::app::AppState;
use crate::prelude::*;

pub async fn login (credential: web::Json<Login>, app_state: web::Data<AppState>) -> Result<HttpResponse, Error> {
    let  credential = credential.into_inner();

    match credential.validate() {
        Ok(_) => (),
        Err(e) => return e,
    };

    let result = app_state.message_handler.send(credential).await;

    match result {
        Ok(res) => Ok(HttpResponse::Ok().json(res)),
        Err(e) => Ok(e.error_response()),
    }
}
