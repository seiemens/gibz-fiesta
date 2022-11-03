/*
----- SUMMARY -----
-> This is basically the Base Layer of the Backend.
*/

extern crate dotenv;

use crate::models::{skill_model::Skill, user_model::User};
use dotenv::dotenv;
use mongodb::{bson::extjson::de::Error, results::InsertOneResult, Client, Collection};
use std::env;

pub struct Connector {
    col: Collection<User>,
}

impl Connector {
    /*
    ----- INITIALIZATION OF DB CONNECTION -----
    */
    pub async fn init() -> Self {
        dotenv().ok();
        //change the var 'key' to change the uri (check your .env file)
        let uri = env::var("MONGOURI").expect("MONGOURI HAS TO BE SET");

        let client = Client::with_uri_str(uri).await.unwrap();
        let db = client.database("fiesta");
        let col: Collection<User> = db.collection("User");
        Connector { col }
    }
}

/*
----- FUNCTION IMPLEMENTATIONS -----
*/

/*
----- USER - RELATED FUNCTIONS -----
*/
impl Connector {
    //insert a new user into the DB
    pub async fn create_user(&self, u: User) -> Result<InsertOneResult, Error> {
        let new = User {
            id: None,
            name: u.name,
            username: u.username,
            email: u.email,
            role: u.role,
            auth_token: u.auth_token,
            completed_skills: Vec::new(),
            password: u.password,
        };
        let user = self
            .col
            .insert_one(new, None)
            .await
            .ok()
            .expect("Error creating user");
        Ok(user)
    }
}
