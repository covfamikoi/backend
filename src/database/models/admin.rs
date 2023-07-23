use crate::database::DbClient;

#[derive(rocket::serde::Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Admin {
    pub id: i32,
    pub username: String,
    #[serde(skip_serializing)]
    pub password_hash: String,
    #[serde(skip_serializing)]
    pub session_version: i32,
    pub superuser: bool,
}

impl Admin {
    pub async fn get_by_username(db: &DbClient, username: &str) -> sqlx::Result<Self> {
        sqlx::query_as!(Self, "select * from admins where username=$1", username)
            .fetch_one(&db.pool)
            .await
    }

    pub async fn session_version(db: &DbClient, id: i32) -> sqlx::Result<i32> {
        sqlx::query_scalar!("select session_version from admins where id=$1", id)
            .fetch_one(&db.pool)
            .await
    }

    pub async fn insert(
        db: &DbClient,
        username: &str,
        password_hash: &str,
        superuser: bool,
    ) -> sqlx::Result<i32> {
        sqlx::query_scalar!(
            "insert into admins (username, password_hash, superuser) values ($1, $2, $3)
            returning id",
            username,
            password_hash,
            superuser
        )
        .fetch_one(&db.pool)
        .await
    }

    pub async fn increment_session_version(
        db: &DbClient,
        id: i32,
    ) -> sqlx::Result<sqlx::postgres::PgQueryResult> {
        sqlx::query!(
            "update admins set session_version=session_version+1 where id=$1",
            id
        )
        .execute(&db.pool)
        .await
    }

    pub async fn set_password_hash(
        db: &DbClient,
        id: i32,
        password_hash: &str,
    ) -> sqlx::Result<sqlx::postgres::PgQueryResult> {
        sqlx::query!(
            "update admins set password_hash=$1, session_version=session_version+1 where id=$2",
            password_hash,
            id
        )
        .execute(&db.pool)
        .await
    }
}
