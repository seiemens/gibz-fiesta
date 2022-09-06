use crate::models::User;
use rocket::{response::status, serde::json::Json};

/*
--- IMPORTANT NOTICE ---
    auth_token: used for API authentication to prohibit access from unauthorized sources.
*/

// [ADMIN] - Register new account
#[post("/n", format = "application/json", data = "<u>")]
pub fn new_user(u: Json<User<'_>>) -> status::Accepted<&str> {
    status::Accepted(Some(u.name))
}
