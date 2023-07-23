use rocket::{
    http::Status,
    request::{FromRequest, Outcome},
    Request,
};

use crate::database::{Admin, DbClient};

use super::verify_password;

pub struct AdminLoginGuard(pub Admin);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AdminLoginGuard {
    type Error = ();

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let db: &DbClient = req.rocket().state().unwrap();

        let username = req.headers().get_one("username");
        let password = req.headers().get_one("password");

        let (username, password) = match (username, password) {
            (Some(username), Some(password)) => (username, password),
            _ => return Outcome::Failure((Status::BadRequest, ())),
        };

        let user = match Admin::get_by_username(db, username).await {
            Ok(user) => user,
            Err(why) => {
                return match why {
                    sqlx::Error::RowNotFound => Outcome::Failure((Status::Unauthorized, ())),
                    _ => Outcome::Failure((Status::InternalServerError, ())),
                }
            }
        };

        match verify_password(password, &user.password_hash) {
            Err(_) => Outcome::Failure((Status::InternalServerError, ())),
            Ok(false) => Outcome::Failure((Status::Unauthorized, ())),
            Ok(true) => Outcome::Success(Self(user)),
        }
    }
}
