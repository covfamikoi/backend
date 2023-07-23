use rocket::{State, serde::json::Json, http::Status};

use crate::{auth::ConferenceViewGuard, database::{DbClient, Event}};

#[get("/conferences/events")]
pub async fn get_events(
    db: &State<DbClient>,
    conf: ConferenceViewGuard,
) -> Result<Json<Vec<Event>>, Status> {
    Event::list_by_conference(db, conf.0.id)
        .await
        .map(Json)
        .map_err(|_| Status::InternalServerError)
}
