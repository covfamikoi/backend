use rocket::{form::Form, http::Status, serde::json::Json, State};

use crate::{
    auth::{ConferenceManageGuard, ConferenceViewGuard},
    database::{Announcement, DbClient},
};

#[get("/conferences/announcements")]
pub async fn get_announcements(
    db: &State<DbClient>,
    conf: ConferenceViewGuard,
) -> Result<Json<Vec<Announcement>>, Status> {
    Announcement::list_by_conference(db, conf.0.id)
        .await
        .map(Json)
        .map_err(|_| Status::InternalServerError)
}

#[derive(FromForm)]
pub struct AnnouncementForm<'r> {
    title: &'r str,
    content: &'r str,
}

#[post("/conferences/announcements", data = "<form>")]
pub async fn post_announcement(
    form: Form<AnnouncementForm<'_>>,
    db: &State<DbClient>,
    conf: ConferenceManageGuard,
) -> Status {
    match Announcement::insert(
        db,
        conf.conference.id,
        conf.admin.id,
        form.title,
        form.content,
    )
    .await
    {
        Ok(_) => Status::Ok,
        Err(_) => Status::InternalServerError,
    }
}

#[patch("/conferences/announcements/<id>", data = "<form>")]
pub async fn patch_announcement(
    form: Form<AnnouncementForm<'_>>,
    db: &State<DbClient>,
    conf: ConferenceManageGuard,
    id: i32,
) -> Status {
    match Announcement::update(db, id, conf.conference.id, form.title, form.content).await {
        Ok(_) => Status::Ok,
        Err(_) => Status::InternalServerError,
    }
}

#[delete("/conferences/announcements/<id>")]
pub async fn delete_announcement(
    db: &State<DbClient>,
    conf: ConferenceManageGuard,
    id: i32,
) -> Status {
    match Announcement::delete(db, id, conf.conference.id).await {
        Ok(_) => Status::Ok,
        Err(_) => Status::InternalServerError,
    }
}
