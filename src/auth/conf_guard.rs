use rocket::{
    http::Status,
    request::{FromRequest, Outcome},
    Request,
};

use crate::database::{Conference, DbClient};

use super::AdminClaims;

async fn conf_from_req(req: &Request<'_>) -> Result<(Conference, bool, Option<AdminClaims>), Status> {
    let db: &DbClient = req.rocket().state().unwrap();

    let Some(Ok(conf_id)) = req.headers().get_one("conference_id").map(|id| id.parse::<i32>()) else {
        return Err(Status::BadRequest);
    };

    let conf = match Conference::get_by_id(db, conf_id).await {
        Ok(conf) => conf,
        Err(why) => {
            return match why {
                sqlx::Error::RowNotFound => Err(Status::NotFound),
                _ => Err(Status::InternalServerError),
            }
        }
    };

    if let Some(admin) = AdminClaims::from_request(req).await.succeeded() {
        if admin.superuser || conf.admins.contains(&admin.id) {
            return Ok((conf, true, Some(admin)));
        }
    }

    let password = req.headers().get_one("conference_password");
    if password == conf.password.as_deref() && password.is_some() {
        return Ok((conf, false, None));
    }

    Err(Status::Unauthorized)
}

pub struct ConferenceManageGuard {
    pub conference: Conference,
    pub admin: AdminClaims
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for ConferenceManageGuard {
    type Error = ();

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, ()> {
        match conf_from_req(req).await {
            Ok((conference, true, Some(admin))) => Outcome::Success(Self { conference, admin }),
            Ok(_) => Outcome::Failure((Status::Unauthorized, ())),
            Err(why) => Outcome::Failure((why, ())),
        }
    }
}

pub struct ConferenceViewGuard(pub Conference);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for ConferenceViewGuard {
    type Error = ();

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, ()> {
        match conf_from_req(req).await {
            Ok((conf, _, _)) => Outcome::Success(Self(conf)),
            Err(why) => Outcome::Failure((why, ())),
        }
    }
}
