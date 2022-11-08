mod api;
mod data;
mod helpers;
mod models;

#[macro_use]
extern crate rocket;

//add imports below
use api::user::{create_user, login_user};
use data::mongo_connector::Connector;

#[launch]
async fn rocket() -> _ {
    let db = Connector::init().await;
    // .manage() -> makes the db accessible in other files.
    rocket::build()
        .manage(db)
        .mount("/", routes![create_user, login_user])
}
