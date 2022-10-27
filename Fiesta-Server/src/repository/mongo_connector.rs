/*
----- SUMMARY -----
-> This is basically the Base Layer of the Backend. No real "logic" here, just inserting, editing and removing data
*/

extern crate dotenv;

use crate::models::user_model::User;
use dotenv::dotenv;
use mongodb::{
    bson::extjson::de::Error,
    results::InsertOneResult,
    sync::{Client, Collection},
};
use std::env;

pub struct Database {
    col: Collection<User>,
}

impl Database {
    /*
    ----- INITIALIZATION OF DB CONNECTION -----
    */
    pub fn init() -> Self {
        dotenv().ok();
        //change the var 'key' to change the uri (check your .env file)
        let uri = match env::var("MONGOURI") {
            Ok(v) => v.to_string(),
            Err(_) => format!("Error loading env variable"),
        };

        let client = Client::with_uri_str(uri).unwrap();
        let db = client.database("fiesta");
        let col: Collection<User> = db.collection("User");
        Database { col }
    }

    /*
    ----- DATABASE LAYER: FUNCTION IMPLEMENTATIONS -----
    */

    //insert a new user into the DB
    pub fn create_user(&self, u: User) -> Result<InsertOneResult, Error> {
        let new = User {
            id: None,
            name: u.name,
            email: u.email,
            role: u.role,
            auth_token: u.auth_token,
            completed_skills: 0,
        };
        let user = self
            .col
            .insert_one(new, None)
            .ok()
            .expect("Error creating user");
        Ok(user)
    }
}
