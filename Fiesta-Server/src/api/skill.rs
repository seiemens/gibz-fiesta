use crate::{
    data::{self, mongo_connector::Connector},
    helpers::{endecr, grandmas_bakery::biscuit, token},
    models::{skill_model::Skill, user_model::User},
};
use argon2::Error;
use mongodb::results::InsertOneResult;
use rocket::{
    http::{Cookie, CookieJar, Status},
    response::content,
    serde::json::{serde_json, Json},
    Request, Response, State,
};
use serde::{Deserialize, Serialize};

/// NON - ENDPOINT related. Used to filter out / sort User form data easier.
pub fn get_skill_data(s: Json<Skill>) -> Result<Skill, Error> {
    let data = Skill {
        name: s.name.to_owned(),
        recommended_group: s.recommended_group.to_owned(),
        subcategories: s.subcategories.to_owned(),
    };
    return Ok(data);
}

#[post("/skill/create", data = "<s>")]
pub async fn create_skill(
    db: &State<Connector>,
    s: Json<Skill>,
) -> Result<Json<InsertOneResult>, Status> {
    let data = get_skill_data(s).unwrap();
    let result = db.create_skill(data).await;
    match result {
        Ok(skill) => Ok(Json(skill)),
        Err(_) => Err(Status::ImATeapot),
    }
}

#[post("/skill/complete", data = "<s>")]
pub async fn mark_skill(db: &State<Connector>, s: Json<Skill>) -> Result<Json<bool>, Status> {}

// // Mark skill as completed / not completed
// #[post("/c", format = "application/json", data = "<u>")]
// pub fn chk_skill(u: Json<Skill>) -> status::Accepted<&'static str> {
//     status::Accepted(Some("HEHEE"))
// }

// // [ADMIN] - Create new Skill
// #[post("/p", format = "application/json", data = "<u>")]
// pub fn post_skill(u: Json<Skill>) -> status::Accepted<&'static str> {
//     status::Accepted(Some("HEHEE"))
// }

// // [ADMIN] - Delete Skill
// #[post("/d", format = "application/json", data = "<u>")]
// pub fn delete_skill(u: Json<Skill>) -> status::Accepted<&'static str> {
//     status::Accepted(Some("HEHEE"))
// }

// // [ADMIN] - Create new Skill
// #[post("/n", format = "application/json", data = "<u>")]
// pub fn new_skill(u: Json<Skill>) -> status::Accepted<&'static str> {
//     status::Accepted(Some("HEHEE"))
// }

// // [ADMIN] - Create new Skill
// #[post("/u", format = "application/json", data = "<u>")]
// pub fn upd_skill(u: Json<Skill>) -> status::Accepted<&'static str> {
//     status::Accepted(Some("HEHEE"))
// }
