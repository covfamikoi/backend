pub struct DbClient {
    // todo: caching?
    pub pool: sqlx::PgPool,
}

impl DbClient {
    pub async fn connect(url: &str) -> anyhow::Result<Self> {
        let pool = sqlx::PgPool::connect(url).await?;
        sqlx::migrate!().run(&pool).await?;

        Ok(Self { pool })
    }
}
