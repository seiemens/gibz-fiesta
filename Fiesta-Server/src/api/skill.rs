use crate::{
    data::mongo_connector::Connector,
    helpers::{endecr, grandmas_bakery::get_biscuit_recipe, token},
    models::{skill_model::Skill, user_model::User},
};
use argon2::Error;
use mongodb::results::{InsertOneResult, UpdateResult};
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
        _id: s._id.to_owned(),
        name: s.name.to_owned(),
        levels: s.levels.to_owned(),
    };
    return Ok(data);
}

#[post("/skill/create", data = "<s>")]
pub async fn create_skill(
    db: &State<Connector>,
    jar: &CookieJar<'_>,
    s: Json<Skill>,
) -> Result<Json<InsertOneResult>, Status> {
    let auth_token = get_biscuit_recipe(jar, "auth_biscuit".to_string());
    if db.verify_auth(auth_token.to_owned()).await == Err(false) {
        return Err(Status::Forbidden);
    } else {
        let data = get_skill_data(s).unwrap();
        let result = db.create_skill(data).await;
        match result {
            Ok(skill) => Ok(Json(skill)),
            Err(_) => Err(Status::ImATeapot),
        }
    }
}

#[post("/skill/complete", data = "<s>")]
pub async fn complete_skill(
    db: &State<Connector>,
    jar: &CookieJar<'_>,
    s: Json<Skill>,
) -> Result<Json<UpdateResult>, Status> {
    let data = get_skill_data(s).unwrap();
    let auth = get_biscuit_recipe(jar, String::from("auth_biscuit"));

    let result = db.complete_skill(data._id.unwrap(), auth).await;

    match result {
        Ok(skill) => Ok(Json(skill)),
        Err(_) => Err(Status::ImATeapot),
    }
}

#[post("/skill/mark", data = "<s>")]
pub async fn mark_skill(
    db: &State<Connector>,
    jar: &CookieJar<'_>,
    s: Json<Skill>,
) -> Result<Status, Status> {
    let data = get_skill_data(s).unwrap();
    let auth = get_biscuit_recipe(jar, String::from("auth_biscuit"));

    let result = db.mark_skill(data._id.unwrap(), auth).await;

    match result {
        Ok(skill) => Ok(Status::Accepted),
        Err(_) => Err(Status::ImATeapot),
    }
}

#[get("/skill/all")]
pub async fn get_all_skills(
    jar: &CookieJar<'_>,
    db: &State<Connector>,
) -> Result<Json<Vec<Skill>>, Status> {
    let data = db.get_skills().await;
    return Ok(Json(data.unwrap()));
}

#[post("/skill/delete", data = "<s>")]
pub async fn delete_skill(
    db: &State<Connector>,
    jar: &CookieJar<'_>,
    s: Json<Skill>,
) -> Result<Status, Status> {
    //authenticate user
    let auth_token = get_biscuit_recipe(jar, "auth_biscuit".to_string());
    if db.verify_auth(auth_token.to_owned()).await == Err(false) {
        return Err(Status::Forbidden);
    } else {
        let data = get_skill_data(s).unwrap();

        let res = db.delete_skill(data).await.unwrap();
        //send error if not modifiable / not found
        if res.deleted_count == 0 {
            return Err(Status::ImATeapot);
        } else {
            return Ok(Status::Accepted);
        }
    }
}

#[post("/skill/update", data = "<s>")]
pub async fn update_skill(
    db: &State<Connector>,
    jar: &CookieJar<'_>,
    s: Json<Skill>,
) -> Result<Json<InsertOneResult>, Status> {
    let data = get_skill_data(s).unwrap();
    let auth = get_biscuit_recipe(jar, String::from("auth_biscuit"));

    let result = db.update_skill(data, auth).await;

    match result {
        Ok(skill) => Ok(Json(skill)),
        Err(_) => Err(Status::ImATeapot),
    }
}
