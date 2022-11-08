/*
--- IMPORTANT NOTICE ---
    auth_token: used for API authentication to prohibit access from unauthorized sources.
*/
use crate::{
    data::mongo_connector::Connector,
    helpers::{biscuit::biscuit, endecr, token},
    models::{skill_model::Skill, user_model::User},
};
use argon2::Error;
use mongodb::results::InsertOneResult;
use rocket::{
    http::{Cookie, CookieJar, Status},
    serde::json::Json,
    Request, Response, State,
};

/// NON - ENDPOINT related. Used to filter out / sort User form data easier.
pub fn get_user_data(u: Json<User>) -> Result<User, Error> {
    let data = User {
        name: u.name.to_owned(),
        username: u.username.to_owned(),
        password: endecr::encrypt(u.password.to_owned()),
        email: u.email.to_owned(),
        role: u.role.to_owned(),
        field: u.field.to_owned(),
        completed_skills: Some(Vec::<Skill>::new()),
        auth_token: Some(token::generate(64)),
        active: Some(true),
    };
    return Ok(data);
}

#[post("/user/create", data = "<u>")]
pub async fn create_user(
    db: &State<Connector>,
    u: Json<User>,
) -> Result<Json<InsertOneResult>, Status> {
    let data = get_user_data(u).unwrap();
    let user_detail = db.create_user(data).await;
    match user_detail {
        Ok(user) => Ok(Json(user)),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[post("/user/login", data = "<u>")]
pub async fn login_user(
    jar: &CookieJar<'_>,
    db: &State<Connector>,
    u: Json<User>,
) -> Result<Status, Status> {
    let data = get_user_data(u).unwrap();
    let user = db.get_user(data).await;

    if user.username.len() > 1 {
        jar.add(biscuit(
            String::from("auth"),
            String::from(user.auth_token.unwrap()),
        ));
        return Ok(Status::Accepted);
    } else {
        return Err(Status::ImATeapot);
    }
}

/*
--- GENERAL ROUTES ---
*/

// Register new account
// #[post("/register", format = "application/json", data = "<u>")]
// pub fn new_user(u: Json<User<'_>>) -> status::Accepted<&str> {
//     status::Accepted(Some(u.name))
// }

// #[post("/login", format = "application/json", data = "<u>")]
// pub fn login_user(u: Json<User<'_>>) -> status::Accepted<&str> {
//     status::Accepted(Some(u.name))
// }

/*
--- ADMIN ROUTES ---
*/

// [ADMIN] - Get all users in list for mgmt
// #[get("/getUser")]
// pub fn get_users() -> status::Accepted<&'static str> {
//     status::Accepted(Some("yay"))
// }

// // [ADMIN] - Update user
// #[post("/setUser", format = "application/json", data = "<u>")]
// pub fn set_user(u: Json<User<'_>>) -> status::Accepted<&str> {
//     status::Accepted(Some(u.name))
// }

// // [ADMIN] - Delete user
// #[post("/delUser", format = "application/json", data = "<u>")]
// pub fn del_user(u: Json<User<'_>>) -> status::Accepted<&str> {
//     status::Accepted(Some(u.name))
// }

// // [ADMIN] - Deactivate user
// #[post("/deactivateUser", format = "application/json", data = "<u>")]
// pub fn deac_user(u: Json<User<'_>>) -> status::Accepted<&str> {
//     status::Accepted(Some(u.name))
// }
