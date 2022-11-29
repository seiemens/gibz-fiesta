/*
--- IMPORTANT NOTICE ---
    auth_token: used for API authentication to prohibit access from unauthorized sources.
*/
use crate::{
    data::{self, mongo_connector::Connector},
    helpers::{
        endecr,
        grandmas_bakery::{biscuit, get_biscuit_recipe},
        token,
    },
    models::{
        skill_model::{Skill, SubSkill},
        user_model::User,
    },
};
use argon2::Error;
use mongodb::results::InsertOneResult;
use rocket::{
    http::{Cookie, CookieJar, Status},
    response::content,
    serde::json::Json,
    Request, Response, State,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LoginData {
    username: String,
}

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
        marked_skills: Some(Vec::<Skill>::new()),
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
        Err(_) => Err(Status::ImATeapot),
    }
}

#[post("/user/login", data = "<u>")]
pub async fn login_user(
    jar: &CookieJar<'_>,
    db: &State<Connector>,
    u: Json<User>,
) -> Result<Json<User>, Status> {
    let data = get_user_data(u).unwrap();
    let user = db.get_user(data).await;

    if let Ok(None) = user {
        return Err(Status::ImATeapot);
    } else {
        let temp = user.clone();
        jar.add(biscuit(
            String::from("auth_biscuit"),
            String::from(user.unwrap().unwrap().auth_token.unwrap()),
        ));
        return Ok(Json(temp.unwrap().unwrap()));
    }
}

#[get("/user/logout")]
pub fn logout_user(jar: &CookieJar<'_>) -> Result<Status, Status> {
    jar.remove(Cookie::named("auth_biscuit"));
    return Ok(Status::Ok);
}

#[post("/user/update", data = "<u>")]
pub async fn update_user(
    jar: &CookieJar<'_>,
    db: &State<Connector>,
    u: Json<User>, //contains username and new pw
) -> Result<Status, Status> {
    //extract cookie from request and process it
    let auth_token = get_biscuit_recipe(jar, String::from("auth_biscuit"));

    let data = get_user_data(u).unwrap();

    //authenticate user && check if pw isn't none
    if db.verify_auth(auth_token.to_owned()).await == Err(false) {
        return Err(Status::Forbidden);
    } else {
        let res = db.update_user(data.username, data.password).await.unwrap();
        //send error if not modifiable / not found
        if res.modified_count == 0 || res.matched_count == 0 {
            return Err(Status::ImATeapot);
        } else {
            return Ok(Status::Accepted);
        }
    }
}

#[post("/user/delete", data = "<u>")]
pub async fn delete_user(
    jar: &CookieJar<'_>,
    db: &State<Connector>,
    u: Json<LoginData>,
) -> Result<Status, Status> {
    let auth_token = get_biscuit_recipe(jar, String::from("auth_biscuit"));
    let data = u.username.to_owned();

    //authenticate user
    if db.verify_auth(auth_token.to_owned()).await == Err(false) {
        return Err(Status::Forbidden);
    } else {
        let res = db.delete_user(data).await.unwrap();
        //send error if not modifiable / not found
        if res.deleted_count == 0 {
            return Err(Status::ImATeapot);
        } else {
            return Ok(Status::Accepted);
        }
    }
}

#[get("/user/auth")]
pub async fn auth_user(
    jar: &CookieJar<'_>,
    db: &State<Connector>,
) -> Result<Json<User>, Json<bool>> {
    let auth = db
        .verify_auth(get_biscuit_recipe(jar, String::from("auth_biscuit")))
        .await;
    if auth.is_ok() {
        return Ok(Json(auth.unwrap()));
    } else {
        return Err(Json(false));
    }
}

#[get("/user/all")]
pub async fn get_all(
    jar: &CookieJar<'_>,
    db: &State<Connector>,
) -> Result<Json<Vec<User>>, Status> {
    let auth = db
        .verify_admin(get_biscuit_recipe(jar, String::from("auth_biscuit")))
        .await;

    if auth.is_ok() {
        let data = db.get_users().await;
        return Ok(Json(data.unwrap()));
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