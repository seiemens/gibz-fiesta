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

// TODO: Implement auth verification
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
pub async fn complete_skill(db: &State<Connector>, s: Json<Skill>) -> Result<Json<bool>, Status> {}

#[post("/skill/mark", data = "<s>")]
pub async fn mark_skill(db: &State<Connector>, s: Json<Skill>) -> Result<Json<bool>, Status> {}

#[post("/skill/delete", data = "<s>")]
pub async fn delete_skill(db: &State<Connector>, s: Json<Skill>) -> Result<Json<bool>, Status> {}

#[post("/skill/update", data = "<s>")]
pub async fn update_skill(db: &State<Connector>, s: Json<Skill>) -> Result<Json<bool>, Status> {}
