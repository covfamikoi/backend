use crate::database::DbClient;

#[derive(rocket::serde::Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Conference {
    pub id: i32,

    pub title: String,
    pub info: String,
    pub password: Option<String>,
    // Option::None => private, probably a preset, only visible to admins with access
    // Option::Some(_) => public
    pub start_ts: chrono::DateTime<chrono::Utc>,
    pub end_ts: chrono::DateTime<chrono::Utc>,

    pub top_left_lat: f64,
    pub top_left_lon: f64,
    pub width_in_tiles: i16,
    pub height_in_tiles: i16,

    pub admins: Vec<i32>,
}

impl Conference {
    pub async fn get_by_id(db: &DbClient, id: i32) -> sqlx::Result<Self> {
        sqlx::query_as!(Self, "select * from conferences where id=$1", id)
            .fetch_one(&db.pool)
            .await
    }

    pub async fn get_public_upcoming(db: &DbClient) -> sqlx::Result<Vec<Self>> {
        sqlx::query_as!(
            Self,
            "select * from conferences where password is not null order by start_ts"
        )
        .fetch_all(&db.pool)
        .await
    }

    pub async fn get_admin_managed(db: &DbClient, admin_id: i32) -> sqlx::Result<Vec<Self>> {
        sqlx::query_as!(
            Self,
            "select * from conferences where $1=any(admins) order by start_ts",
            admin_id
        )
        .fetch_all(&db.pool)
        .await
    }

    pub async fn get_all(db: &DbClient) -> sqlx::Result<Vec<Self>> {
        sqlx::query_as!(Self, "select * from conferences order by start_ts")
            .fetch_all(&db.pool)
            .await
    }

    pub async fn insert_empty(db: &DbClient, admins: &[i32]) -> sqlx::Result<i32> {
        sqlx::query_scalar!(
            "insert into conferences (admins) values ($1) returning id",
            admins,
        )
        .fetch_one(&db.pool)
        .await
    }

    pub async fn delete(db: &DbClient, id: i32) -> sqlx::Result<sqlx::postgres::PgQueryResult> {
        sqlx::query!("delete from conferences where id=$1", id)
        .execute(&db.pool)
        .await
    }

    pub async fn update(&self, db: &DbClient) -> sqlx::Result<sqlx::postgres::PgQueryResult> {
        sqlx::query!(
            "update conferences set
            title=$1, info=$2, password=$3, start_ts=$4, end_ts=$5, top_left_lat=$6,
            top_left_lon=$7, width_in_tiles=$8, height_in_tiles=$9, admins=$10
            where id=$11",
            self.title,
            self.info,
            self.password,
            self.start_ts,
            self.end_ts,
            self.top_left_lat,
            self.top_left_lon,
            self.width_in_tiles,
            self.height_in_tiles,
            &self.admins,
            self.id
        )
        .execute(&db.pool)
        .await
    }
}
