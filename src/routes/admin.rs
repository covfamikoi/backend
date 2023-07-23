use rocket::{form::Form, http::Status, State};

use crate::{
    auth::{new_password, AdminClaims, AdminLoginGuard},
    config::Config,
    database::{Admin, DbClient},
};

#[post("/admin/login")]
pub async fn login(config: &State<Config>, user: AdminLoginGuard) -> String {
    let jwt = AdminClaims::new(user.0.id, user.0.superuser, user.0.session_version);
    jwt.gen_jwt_token(config.jwt_secret.as_bytes())
}

#[post("/admin/logout")]
pub async fn logout(db: &State<DbClient>, user: AdminLoginGuard) -> Status {
    match Admin::increment_session_version(db, user.0.id).await {
        Ok(_) => Status::Ok,
        Err(_) => Status::InternalServerError,
    }
}

#[derive(FromForm)]
pub struct EditAdminPassword<'r> {
    new_password: &'r str,
}

#[post("/admin/edit-password", data = "<form>")]
pub async fn edit_password(
    db: &State<DbClient>,
    admin: AdminLoginGuard,
    form: Form<EditAdminPassword<'_>>,
) -> Status {
    let Ok(pwd_hash) = new_password(form.new_password) else {
        return Status::InternalServerError;
    };

    let Ok(_) = Admin::set_password_hash(db, admin.0.id, &pwd_hash).await else {
        return Status::InternalServerError;
    };

    Status::Ok
}

#[derive(FromForm)]
pub struct CreateAdmin<'r> {
    username: &'r str,
    password: &'r str,
    superuser: bool,
}

#[post("/admin/create", data = "<form>")]
pub async fn create_admin(
    db: &State<DbClient>,
    admin: AdminLoginGuard,
    form: Form<CreateAdmin<'_>>,
) -> Status {
    if form.superuser && !admin.0.superuser {
        return Status::Forbidden;
    }

    let Ok(pwd_hash) = new_password(form.password) else {
        return Status::InternalServerError;
    };

    let Ok(_) = Admin::insert(db, form.username, &pwd_hash, form.superuser).await else {
        return Status::InternalServerError;
    };

    Status::Ok
}
