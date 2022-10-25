//include routes
mod models;
mod routes;

//use em
use models::{Skill, User};
use mongodb::{
    bson::doc,
    options::{ClientOptions, ResolverConfig},
    Client, Collection, Database,
};
use rocket::serde::{Deserialize, Serialize};
use rocket_contrib::json::Json;
use routes::{skills, users};
use std::error::Error;
use tokio;
use users::*;

#[macro_use]
extern crate rocket;

async fn connect_to_db() -> Result<(), Box<dyn Error>> {
    // Load the MongoDB connection string from an environment variable:
    let client_uri = "mongodb://localhost:27017";
    let options =
        ClientOptions::parse_with_resolver_config(&client_uri, ResolverConfig::cloudflare())
            .await?;
    let client = Client::with_options(options)?;
    let db = client.database("fiesta");

    // Select Collection AND COLLECTION TYPE!!!
    let user_col: Collection<User> = db.collection("users");
    let skills_col: Collection<Skill> = db.collection("skills");

    //ok.
    Ok(())
}

#[get("/")]
fn index() -> &'static str {
    "yay."
}

#[launch]
async fn rocket() -> _ {
    connect_to_db().await;
    rocket::build().mount("/", routes![index, get_users])
}
