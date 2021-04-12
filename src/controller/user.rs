use actix_web::{HttpResponse, web, Result};
use crate::message::user::{Login, Register};
use crate::app::AppState;
use crate::error::Error;
use validator::Validate;
use serde_json::json;

pub async fn login (credential: web::Json<Login>, app_state: web::Data<AppState>) -> Result<HttpResponse, Error> {
    let credential = credential.into_inner();
    credential.validate()?;

    let response = app_state.message_handler.send(credential).await?;
    match response {
        Err(_error) =>  Ok(
            HttpResponse::Ok()
            .json(json!({
                "message" : "用户或密码错误"
            }))
        ),
        Ok(_data) => {
            Ok(
                HttpResponse::Ok()
                .json(json!({
                    "token" : "Okay"
                }))
            )
        }
    }
}

pub async fn register(registration: web::Json<Register>, app_state: web::Data<AppState>) -> Result<HttpResponse, Error> {
    let registration = registration.into_inner();
    registration.validate()?;

    let response = app_state.message_handler.send(registration).await?;
    match response {
        Err(_error) =>  Ok(
            HttpResponse::Ok()
            .json(json!({
                "message" : "用户或密码错误"
            }))
        ),
        Ok(data) => {
            Ok(
                HttpResponse::Ok()
                .json(json!(data))
            )
        }
    }
}