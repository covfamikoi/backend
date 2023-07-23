use rocket::{
    http::Status,
    request::{FromRequest, Outcome},
    serde::{Deserialize, Serialize},
    Request,
};

use crate::{config::Config, database::{DbClient, Admin}};

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct AdminClaims {
    pub exp: usize,
    pub id: i32,
    pub superuser: bool,
    pub session_version: i32,
}

impl AdminClaims {
    pub fn new(id: i32, superuser: bool, session_version: i32) -> Self {
        Self {
            exp: chrono::Utc::now()
                .checked_add_days(chrono::Days::new(14))
                .unwrap()
                .timestamp() as usize,
            id,
            superuser,
            session_version
        }
    }

    pub fn gen_jwt_token(&self, jwt_secret: &[u8]) -> String {
        jsonwebtoken::encode(
            &jsonwebtoken::Header::default(),
            self,
            &jsonwebtoken::EncodingKey::from_secret(jwt_secret),
        )
        .unwrap()
    }

    pub fn from_token(token: &str, jwt_secret: &[u8]) -> Option<Self> {
        let token = jsonwebtoken::decode(
            token,
            &jsonwebtoken::DecodingKey::from_secret(jwt_secret),
            &jsonwebtoken::Validation::default(),
        );

        token.ok().map(|v| v.claims)
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AdminClaims {
    type Error = ();

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, ()> {
        let config: &Config = req.rocket().state().unwrap();
        let db: &DbClient = req.rocket().state().unwrap();

        let Some(jwt) = req.headers().get_one("jwt") else {
            return Outcome::Failure((Status::Unauthorized, ()));
        };

        let Some(claims) = Self::from_token(jwt, config.jwt_secret.as_bytes()) else {
            return Outcome::Failure((Status::Unauthorized, ()));
        };

        let Ok(session_version) = Admin::session_version(db, claims.id).await else {
            return Outcome::Failure((Status::InternalServerError, ()));
        };

        if session_version == claims.session_version {
            Outcome::Success(claims)
        } else {
            Outcome::Failure((Status::Unauthorized, ()))
        }
    }
}
