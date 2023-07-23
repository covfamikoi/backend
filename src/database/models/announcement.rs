use crate::database::DbClient;

#[derive(rocket::serde::Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Announcement {
    pub id: i32,
    pub conference_id: i32,

    pub posted_by: i32,

    pub title: String,
    pub content: String,
}

impl Announcement {
    pub async fn insert(
        db: &DbClient,
        conference_id: i32,
        posted_by: i32,
        title: &str,
        content: &str,
    ) -> sqlx::Result<i32> {
        sqlx::query_scalar!(
            "insert into announcements (conference_id, posted_by, title, content)
            values ($1, $2, $3, $4) returning id",
            conference_id,
            posted_by,
            title,
            content
        )
        .fetch_one(&db.pool)
        .await
    }

    pub async fn update(
        db: &DbClient,
        id: i32,
        conference_id: i32,
        title: &str,
        content: &str,
    ) -> sqlx::Result<sqlx::postgres::PgQueryResult> {
        sqlx::query!(
            "update announcements set title=$1, content=$2 where id=$3 and conference_id=$4",
            title,
            content,
            id,
            conference_id
        )
        .execute(&db.pool)
        .await
    }

    pub async fn delete(
        db: &DbClient,
        id: i32,
        conference_id: i32,
    ) -> sqlx::Result<sqlx::postgres::PgQueryResult> {
        sqlx::query!("delete from announcements where id=$1 and conference_id=$2", id, conference_id)
        .execute(&db.pool)
        .await
    }

    pub async fn list_by_conference(db: &DbClient, conference_id: i32) -> sqlx::Result<Vec<Self>> {
        sqlx::query_as!(
            Self,
            "select * from announcements where conference_id=$1",
            conference_id
        )
        .fetch_all(&db.pool)
        .await
    }
}
