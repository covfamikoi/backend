#[derive(rocket::serde::Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Category {
    pub id: i32,
    pub conference_id: i32,

    pub title: String,
    pub info: Option<String>,

    /// 0=no notifications, 1=force notifications, 2=default on, 3=default off
    ///
    /// defaults to 3
    pub notification_preset: i16,
}
