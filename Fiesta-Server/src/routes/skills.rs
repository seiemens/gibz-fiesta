use crate::models::User;
use rocket::{response::status, serde::json::Json};

// Mark skill as completed / not completed
#[post("/checkSkill", format = "application/json", data = "<u>")]
pub fn chk_skill(u: Json<User<'_>>) -> status::Accepted<&str> {
    status::Accepted(Some(u.name))
}

// [ADMIN] - Create new Skill
#[post("/postSkill", format = "application/json", data = "<u>")]
pub fn post_skill(u: Json<User<'_>>) -> status::Accepted<&str> {
    status::Accepted(Some(u.name))
}

// [ADMIN] - Delete Skill
#[post("/delSkill", format = "application/json", data = "<u>")]
pub fn del_skill(u: Json<User<'_>>) -> status::Accepted<&str> {
    status::Accepted(Some(u.name))
}

// [ADMIN] - Create new Skill
#[post("/newSkill", format = "application/json", data = "<u>")]
pub fn new_skill(u: Json<User<'_>>) -> status::Accepted<&str> {
    status::Accepted(Some(u.name))
}

// [ADMIN] - Create new Skill
#[post("/updateSkill", format = "application/json", data = "<u>")]
pub fn upd_skill(u: Json<User<'_>>) -> status::Accepted<&str> {
    status::Accepted(Some(u.name))
}
