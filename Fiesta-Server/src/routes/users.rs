/*
--- IMPORTANT NOTICE ---
    auth_token: used for API authentication to prohibit access from unauthorized sources.
*/

use crate::models::User;
use rocket::{response::status, serde::json::Json};

/*
--- GENERAL ROUTES ---
*/

// Register new account
#[post("/register", format = "application/json", data = "<u>")]
pub fn new_user(u: Json<User<'_>>) -> status::Accepted<&str> {
    status::Accepted(Some(u.name))
}

#[post("/login", format = "application/json", data = "<u>")]
pub fn login_user(u: Json<User<'_>>) -> status::Accepted<&str> {
    status::Accepted(Some(u.name))
}

/*
--- ADMIN ROUTES ---
*/

// [ADMIN] - Get all users in list for mgmt
#[get("/getUser")]
pub fn get_users() -> status::Accepted<&'static str> {
    status::Accepted(Some("yay"))
}

// [ADMIN] - Update user
#[post("/setUser", format = "application/json", data = "<u>")]
pub fn set_user(u: Json<User<'_>>) -> status::Accepted<&str> {
    status::Accepted(Some(u.name))
}

// [ADMIN] - Delete user
#[post("/delUser", format = "application/json", data = "<u>")]
pub fn del_user(u: Json<User<'_>>) -> status::Accepted<&str> {
    status::Accepted(Some(u.name))
}

// [ADMIN] - Deactivate user
#[post("/deactivateUser", format = "application/json", data = "<u>")]
pub fn deac_user(u: Json<User<'_>>) -> status::Accepted<&str> {
    status::Accepted(Some(u.name))
}
