//include routes
mod models;
mod routes;
//use em
use mongodb::bson;
use rocket::serde::{json::Json, Deserialize};
use routes::{skills, users};

#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "yay."
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, users::new_user])
}
