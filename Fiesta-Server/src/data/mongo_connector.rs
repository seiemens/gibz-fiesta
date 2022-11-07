/*
----- SUMMARY -----
-> This is basically the Base Layer of the Backend.
*/

extern crate dotenv;

use crate::{
    helpers::endecr,
    models::{skill_model::Skill, user_model::User},
};
use dotenv::dotenv;
use mongodb::{
    bson::{doc, extjson::de::Error},
    options::{FindOneOptions, FindOptions},
    results::InsertOneResult,
    Client, Collection,
};
use std::env;

pub struct Connector {
    user_col: Collection<User>,
    skill_col: Collection<Skill>,
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
        let user_col: Collection<User> = db.collection("users");
        let skill_col: Collection<Skill> = db.collection("skills");
        Connector {
            user_col,
            skill_col,
        }
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
            name: u.name,
            username: u.username,
            email: u.email,
            role: u.role,
            auth_token: u.auth_token,
            completed_skills: Vec::new(),
            password: u.password,
            active: true,
        };
        let user = self
            .user_col
            .insert_one(new, None)
            .await
            .ok()
            .expect("Error creating user");
        Ok(user)
    }

    pub async fn login_user(&self, u: User) -> Result<bool, Error> {
        let filter = doc! { "username": u.username, "password": u.password };
        let user = self.user_col.find_one(filter, None).await;

        println!("{:?}", user);
        if let Ok(None) = user {
            return Ok(false);
        } else {
            return Ok(true);
        }
    }
}
