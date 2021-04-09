use actix_web::{HttpResponse, web};
use crate::message::user::Login;
use crate::app::AppState;
use crate::prelude::*;
use validator::Validate;

pub async fn login (credential: web::Json<Login>, app_state: web::Data<AppState>) -> Result<HttpResponse> {
    let  credential = credential.into_inner();

    credential.validate()?;
    println!("{:?}", credential);

    let response = app_state.message_handler.send(credential).await?;
    Ok(HttpResponse::Ok().json(response.unwrap()))
}
