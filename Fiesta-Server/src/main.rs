mod api;
mod data;
mod helpers;
mod models;

#[macro_use]
extern crate rocket;

//add imports below
use crate::api::skill::create_skill;
use api::user::{auth_user, create_user, delete_user, login_user, logout_user, update_user};
use data::mongo_connector::Connector;

#[launch]
async fn rocket() -> _ {
    let db = Connector::init().await;
    // .manage() -> makes the db accessible in other files.
    rocket::build().manage(db).mount(
        "/",
        routes![
            create_user,
            login_user,
            logout_user,
            update_user,
            delete_user,
            auth_user,
            create_skill
        ],
    )
}
