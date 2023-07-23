pub struct Config {
    pub jwt_secret: String,
    pub db_url: String,
    pub superuser_pwd: String,
}

impl Config {
    pub fn load() -> Self {
        let _ = dotenv::dotenv();

        Self {
            jwt_secret: std::env::var("JWT_SECRET").expect("JWT_SECRET"),
            db_url: std::env::var("DATABASE_URL").expect("DATABASE_URL"),
            superuser_pwd: std::env::var("SUPERUSER_PWD").expect("SUPERUSER_PWD"),
        }
    }
}
