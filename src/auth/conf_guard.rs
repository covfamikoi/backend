use rocket::{
    http::Status,
    request::{FromRequest, Outcome},
    Request,
};

use crate::database::{Conference, DbClient};

use super::AdminClaims;

enum AdminClaimsResult {
    Ok(AdminClaims),
    Fail(Status),
}

async fn conf_from_req(req: &Request<'_>) -> Result<(Conference, AdminClaimsResult), Status> {
    let db: &DbClient = req.rocket().state().unwrap();

    let Some(Ok(conf_id)) = req.headers().get_one("conference-id").map(|id| id.parse::<i32>()) else {
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

    match AdminClaims::from_request(req).await {
        Outcome::Failure((status, ())) => {
            let admin = AdminClaimsResult::Fail(status);

            let password = req.headers().get_one("conference-password");
            if password == conf.password.as_deref() && password.is_some() {
                Ok((conf, admin))
            } else {
                Err(status)
            }
        }
        Outcome::Success(admin) => Ok((conf, AdminClaimsResult::Ok(admin))),
        _ => unreachable!("no"),
    }
}

pub struct ConferenceManageGuard {
    pub conference: Conference,
    pub admin: AdminClaims,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for ConferenceManageGuard {
    type Error = ();

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, ()> {
        match conf_from_req(req).await {
            Ok((conference, AdminClaimsResult::Ok(admin))) => {
                match conference.admins.contains(&admin.id) {
                    true => Outcome::Success(Self { conference, admin }),
                    false => Outcome::Failure((Status::Forbidden, ())),
                }
            }
            Ok((_, AdminClaimsResult::Fail(status))) | Err(status) => {
                Outcome::Failure((status, ()))
            }
        }
    }
}

pub struct ConferenceViewGuard(pub Conference);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for ConferenceViewGuard {
    type Error = ();

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, ()> {
        dbg!(req);
        match conf_from_req(req).await {
            Ok((conf, _)) => Outcome::Success(Self(conf)),
            Err(why) => Outcome::Failure((why, ())),
        }
    }
}
