#[macro_use]
extern crate rocket;

use auth::new_password;
use config::Config;
use database::Admin;
use database::DbClient;

mod auth;
mod config;
mod database;
mod routes;

#[rocket::main]
async fn main() -> anyhow::Result<()> {
    let config = Config::load();
    let db = DbClient::connect(&config.db_url).await?;

    // create the superuser account
    match Admin::get_by_username(&db, "superuser").await {
        Ok(_) => (),
        Err(why) => match why {
            sqlx::Error::RowNotFound => {
                let password_hash = new_password(&config.superuser_pwd)?;
                Admin::insert(&db, "superuser", &password_hash, true).await?;
            }
            ot => return Err(ot.into()),
        },
    }

    // launch app
    let _rocket = rocket::build()
        .mount("/api", routes::gen_routes())
        .manage(db)
        .manage(config)
        .launch()
        .await?;

    Ok(())
}
