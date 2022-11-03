use crate::models::skill_model::Skill;
use rocket::{response::status, serde::json::Json};
// Mark skill as completed / not completed
#[post("/c", format = "application/json", data = "<u>")]
pub fn chk_skill(u: Json<Skill>) -> status::Accepted<&'static str> {
    status::Accepted(Some("HEHEE"))
}

// [ADMIN] - Create new Skill
#[post("/p", format = "application/json", data = "<u>")]
pub fn post_skill(u: Json<Skill>) -> status::Accepted<&'static str> {
    status::Accepted(Some("HEHEE"))
}

// [ADMIN] - Delete Skill
#[post("/d", format = "application/json", data = "<u>")]
pub fn del_skill(u: Json<Skill>) -> status::Accepted<&'static str> {
    status::Accepted(Some("HEHEE"))
}

// [ADMIN] - Create new Skill
#[post("/n", format = "application/json", data = "<u>")]
pub fn new_skill(u: Json<Skill>) -> status::Accepted<&'static str> {
    status::Accepted(Some("HEHEE"))
}

// [ADMIN] - Create new Skill
#[post("/u", format = "application/json", data = "<u>")]
pub fn upd_skill(u: Json<Skill>) -> status::Accepted<&'static str> {
    status::Accepted(Some("HEHEE"))
}
