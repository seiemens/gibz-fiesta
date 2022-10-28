mod api;
mod data;
mod models;

#[macro_use]
extern crate rocket;

//add imports below
use api::user::create_user;
use data::mongo_connector::Database;

#[launch]
fn rocket() -> _ {
    let db = Database::init();
    rocket::build().manage(db).mount("/", routes![create_user])
}
