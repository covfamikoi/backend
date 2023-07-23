use crate::database::DbClient;

#[derive(rocket::serde::Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Event {
    pub id: i32,
    pub conference_id: i32,
    pub location_id: i32,

    pub title: String,
    pub info: Option<String>,
    pub categories: Vec<i32>,
}

impl Event {
    pub async fn list_by_conference(db: &DbClient, conference_id: i32) -> sqlx::Result<Vec<Self>> {
        sqlx::query_as!(Self, "select * from events where conference_id=$1", conference_id)
        .fetch_all(&db.pool)
        .await
    }
}
