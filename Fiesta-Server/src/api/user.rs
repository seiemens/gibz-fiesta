/*
--- IMPORTANT NOTICE ---
    auth_token: used for API authentication to prohibit access from unauthorized sources.
*/
use crate::{
    data::mongo_connector::Database,
    models::{skill_model::Skill, user_model::User},
};
use mongodb::results::InsertOneResult;
use rand::{distributions::Alphanumeric, Rng};
use rocket::{http::Status, serde::json::Json, State};

#[post("/user", data = "<u>")]
pub fn create_user(db: &State<Database>, u: Json<User>) -> Result<Json<InsertOneResult>, Status> {
    //generate the token randomly
    let s: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(64)
        .map(char::from)
        .collect();

    let data = User {
        id: None,
        name: u.name.to_owned(),
        username: u.username.to_owned(),
        email: u.email.to_owned(),
        role: u.role.to_owned(),
        auth_token: s,
        completed_skills: Vec::<Skill>::new(),
    };
    let user_detail = db.create_user(data);
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
