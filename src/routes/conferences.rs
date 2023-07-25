use chrono::TimeZone;
use rocket::{form::Form, http::Status, serde::json::Json, State};

use crate::{
    auth::{AdminClaims, ConferenceManageGuard, ConferenceViewGuard},
    database::{Conference, DbClient},
};

#[get("/conferences/list/public")]
pub async fn get_public_conferences(db: &State<DbClient>) -> Result<Json<Vec<(i32, String)>>, Status> {
    match Conference::get_public_upcoming(db).await {
        Ok(events) => Ok(Json(events.into_iter().map(|e| (e.id, e.title)).collect())),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[get("/conferences/list/managed")]
pub async fn get_managed_conferences(
    db: &State<DbClient>,
    admin: AdminClaims,
) -> Result<Json<Vec<Conference>>, Status> {
    let ret = if admin.superuser {
        Conference::get_all(db).await
    } else {
        Conference::get_admin_managed(db, admin.id).await
    };

    ret.map(Json).map_err(|_| Status::InternalServerError)
}

#[get("/conferences")]
pub async fn get_conference(conf: ConferenceViewGuard) -> Result<Json<Conference>, Status> {
    Ok(Json(conf.0))
}

#[post("/conferences")]
pub async fn create_conference(db: &State<DbClient>, admin: AdminClaims) -> Status {
    match Conference::insert_empty(db, &[admin.id]).await {
        Ok(_) => Status::Ok,
        Err(_) => Status::InternalServerError,
    }
}

#[delete("/conferences")]
pub async fn delete_conference(db: &State<DbClient>, conf: ConferenceManageGuard) -> Status {
    match Conference::delete(db, conf.conference.id).await {
        Ok(_) => Status::Ok,
        Err(_) => Status::InternalServerError,
    }
}

#[derive(FromForm)]
pub struct UpdateConference<'r> {
    title: &'r str,
    password: Option<&'r str>,

    start_ts: u64,
    end_ts: u64,

    top_left_lat: f64,
    top_left_lon: f64,
    width_in_tiles: i16,
    height_in_tiles: i16,

    add_admins: Vec<i32>,
    remove_admins: Vec<i32>,
}

#[patch("/conferences", data = "<form>")]
pub async fn update_conference(
    db: &State<DbClient>,
    conf: ConferenceManageGuard,
    form: Form<UpdateConference<'_>>,
) -> Status {
    let mut conf = conf.conference;
    conf.title = form.title.into();
    conf.password = form.password.map(|s| s.into());
    let chrono::LocalResult::Single(start_ts) = chrono::Utc.timestamp_opt(form.start_ts as i64, 0) else {
        return Status::InternalServerError;
    };
    conf.start_ts = start_ts;
    let chrono::LocalResult::Single(end_ts) = chrono::Utc.timestamp_opt(form.end_ts as i64, 0) else {
        return Status::InternalServerError;
    };
    conf.end_ts = end_ts;

    conf.top_left_lat = form.top_left_lat;
    conf.top_left_lon = form.top_left_lon;
    conf.width_in_tiles = form.width_in_tiles;
    conf.height_in_tiles = form.height_in_tiles;

    conf.admins.retain(|id| !form.remove_admins.contains(id));
    conf.admins.extend(form.add_admins.iter());

    match conf.update(db).await {
        Ok(_) => Status::Ok,
        Err(_) => Status::InternalServerError,
    }
}
