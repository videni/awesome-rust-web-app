use futures::{future::result, Future};
use actix_web::{HttpResponse, web::Json, web::Data};
use crate::message::user::LoginUser;
use crate::schema::user::dsl::*;
use crate::db::{DbExecutor};
use actix::prelude::*;
use diesel::prelude::*;

pub async fn login(
    (form, state): (Json<In<LoginUser>>, Data<AppState>),
) -> impl Future<Output = HttpResponse, Error = Error> {
    let login_user = form.into_inner().user;

    
    result(login_user.validate())
        .from_err()
        .and_then(move |_| state.db.send(login_user).from_err())
        .and_then(|res| match res {
            Ok(res) => Ok(HttpResponse::Ok().json(res)),
            Err(e) => Ok(e.error_response()),
        })
}

impl Handler<LoginUser> for DbExecutor {
    type Result = Result<UserResponse>;

    fn handle(&mut self, msg: LoginUser, _: &mut Self::Context) -> Self::Result {

        let provided_password_raw = &msg.password;

        let conn = &self.0.get()?;

        let stored_user: User = user.filter(email.eq(msg.email)).first(conn)?;
        let checker = HashBuilder::from_phc(&stored_user.password)?;

        if checker.is_valid(provided_password_raw) {
            if checker.needs_update(PWD_SCHEME_VERSION) {
                let new_password = HASHER.hash(provided_password_raw)?;
                return match diesel::update(users.find(stored_user.id))
                    .set(password.eq(new_password))
                    .get_result::<User>(conn)
                {
                    Ok(user) => Ok(user.into()),
                    Err(e) => Err(e.into()),
                };
            }
            Ok(stored_user.into())
        } else {
            Err(Error::Unauthorized(json!({
                "error": "Wrong password",
            })))
        }
    }
}