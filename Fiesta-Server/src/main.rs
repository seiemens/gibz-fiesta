#[macro_use]
extern crate rocket;

//add imports below
use crate::api::skill::{
    complete_skill, create_skill, delete_skill, get_all_skills, mark_skill, update_skill,
};
use api::user::{
    auth_user, create_user, delete_user, get_all_users, login_user, logout_user, update_user, get_user_profile,
};
use data::mongo_connector::Connector;
use rocket::fairing::Info;
use rocket::fairing::{Fairing, Kind};
use rocket::http::Header;
use rocket::{Request, Response};

mod api;
mod data;
mod helpers;
mod models;

#[launch]
async fn rocket() -> _ {
    let db = Connector::init().await;

    // .manage() -> makes the db accessible in other files.
    rocket::build()
        .manage(db)
        .mount(
            "/",
            routes![
                create_user,
                login_user,
                logout_user,
                update_user,
                delete_user,
                auth_user,
                create_skill,
                get_all_users,
                get_user_profile,
                get_all_skills,
                mark_skill,
                update_skill,
                complete_skill,
                delete_skill,
            ],
        )
        .attach(Cors)
}

// enable cors for rocket
pub struct Cors;
#[rocket::async_trait]
impl Fairing for Cors {
    fn info(&self) -> Info {
        Info {
            name: "Cross-Origin-Resource-Sharing Fairing",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        /*
        ----- IMPORTANT NOTE -----
        -> For some odd reason we didn't figure out yet, the unit tests don't work if this part is uncommented.
           There does not seem to be a fix for it, just live with it! :D
        */
        response.set_header(Header::new(
            "Access-Control-Allow-Origin",
            _request.headers().get("origin").next().unwrap(),
        ));
        response.set_header(Header::new(
            "Access-Control-Allow-Methods",
            "POST, PATCH, PUT, DELETE, HEAD, OPTIONS, GET",
        ));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}

#[cfg(test)]
mod tests;
