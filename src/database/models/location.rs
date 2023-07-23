#[derive(rocket::serde::Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Location {
    pub id: i32,
    pub conference_id: i32,

    pub lat: f64,
    pub lon: f64,

    pub title: String,
    pub info: Option<String>,
}
