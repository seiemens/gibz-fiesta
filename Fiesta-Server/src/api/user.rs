/*
--- IMPORTANT NOTICE ---
    auth_token: used for API authentication to prohibit access from unauthorized sources.
*/
use crate::{
    data::mongo_connector::Connector,
    helpers::{endecr, token},
    models::{skill_model::Skill, user_model::User},
};
use mongodb::results::InsertOneResult;
use rocket::{http::Status, serde::json::Json, State};

#[post("/user", data = "<u>")]
pub async fn create_user(
    db: &State<Connector>,
    u: Json<User>,
) -> Result<Json<InsertOneResult>, Status> {
    let data = User {
        name: u.name.to_owned(),
        username: u.username.to_owned(),
        email: u.email.to_owned(),
        role: u.role.to_owned(),
        auth_token: token::generate(64),
        completed_skills: Vec::<Skill>::new(),
        password: endecr::encrypt(u.password.to_owned()),
        active: true,
    };
    let user_detail = db.create_user(data).await;
    match user_detail {
        Ok(user) => Ok(Json(user)),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[post("/login", data = "<u>")]
pub async fn login_user(db: &State<Connector>, u: Json<User>) -> Result<Json<bool>, Status> {
    let data = User {
        name: u.name.to_owned(),
        username: u.username.to_owned(),
        email: u.email.to_owned(),
        role: u.role.to_owned(),
        auth_token: token::generate(64),
        completed_skills: Vec::<Skill>::new(),
        password: endecr::encrypt(u.password.to_owned()),
        active: true,
    };
    let user_detail = db.login_user(data).await;
    match user_detail {
        Ok(user) => Ok(Json(user)),
        Err(_) => Err(Status::InternalServerError),
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
