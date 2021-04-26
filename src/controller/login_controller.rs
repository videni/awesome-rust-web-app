use actix_web::{
    HttpResponse, 
    web, 
    Result,
    http::StatusCode
};
use crate::app::AppState;
use crate::error::Error;
use validator::Validate;
use serde_json::json;
use crate::message_handler::login_handler::LoginHandler;
use crate::message_handler::MessageHandler;
use crate::message::Login;

pub async fn login(
    credential: web::Json<Login>, 
    app_state: web::Data<AppState>
) -> Result<HttpResponse, Error> {
    let credential = credential.into_inner();
    credential.validate()?;

    let response = LoginHandler(app_state.db_connection_pool.clone()).handle(credential).await;

    match response {
        Err(_error) =>  Ok(
            HttpResponse::build(StatusCode::BAD_REQUEST)
            .json(json!({
                "message" : "用户或密码错误"
            }))
        ),
        Ok(token) => {
            Ok(
                HttpResponse::Ok()
                .json(json!({
                    "token" : token
                }))
            )
        }
    }
}