use validator::Validate;
use futures::{future::result, Future};
use actix_web::{HttpRequest, HttpResponse, web::Json, ResponseError, web::Data};
use std::convert::From;

#[derive(Debug, Validate, Deserialize)]
pub struct LoginUser {
    #[validate(length(
        min = "6",
        max = "30",
        message = "验证失败: 登录名必须为6-30个字符长"
    ))]
    pub username: String,
    #[validate(length(
        min = "8",
        max = "30",
        message = "验证失败：密码长度必须为8-30个字符"
    ))]
    pub password: String,
}

pub fn login(
    (form, state): (Json<In<LoginUser>>, Data<AppState>),
) -> impl Future<Item = HttpResponse, Error = Error> {
    let login_user = form.into_inner().user;

    result(login_user.validate())
        .from_err()
        .and_then(move |_| state.db.send(login_user).from_err())
        .and_then(|res| match res {
            Ok(res) => Ok(HttpResponse::Ok().json(res)),
            Err(e) => Ok(e.error_response()),
        })
}