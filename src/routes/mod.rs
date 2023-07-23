mod admin;
mod announcements;
mod conferences;
mod events;

pub fn gen_routes() -> Vec<rocket::Route> {
    routes![
        admin::login,
        admin::logout,
        admin::edit_password,
        admin::create_admin,
        announcements::get_announcements,
        announcements::post_announcement,
        announcements::patch_announcement,
        announcements::delete_announcement,
        conferences::get_managed_conferences,
        conferences::get_public_conferences,
        conferences::get_conference,
        conferences::create_conference,
        conferences::delete_conference,
        conferences::update_conference,
        events::get_events,
    ]
}
