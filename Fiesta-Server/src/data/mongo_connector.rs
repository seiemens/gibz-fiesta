/*
----- SUMMARY -----
-> This is basically the Base Layer of the Backend.
*/

extern crate dotenv;

use crate::{
    helpers::biscuit::biscuit,
    models::{skill_model::Skill, user_model::User},
};
use dotenv::dotenv;
use mongodb::{
    bson::doc,
    error::Error,
    options::{FindOneOptions, FindOptions},
    results::InsertOneResult,
    Client, Collection, Cursor,
};
use rocket::http::{Cookie, Status};
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
    /// insert a new user into the DB
    pub async fn create_user(&self, u: User) -> Result<InsertOneResult, Error> {
        let new = User {
            name: u.name,
            username: u.username,
            password: u.password,
            email: u.email,
            role: u.role,
            field: u.field,
            auth_token: u.auth_token,
            completed_skills: u.completed_skills,
            active: u.active,
        };
        let user = self
            .user_col
            .insert_one(new, None)
            .await
            .ok()
            .expect("Error creating user");
        Ok(user)
    }

    /// get user based on password & username
    pub async fn get_user(&self, u: User) -> Result<Option<User>, Error> {
        // println!("{} {}", u.username, u.password);
        let filter = doc! { "username": u.username, "password": u.password };
        let result = self.user_col.find_one(filter, None).await?;
        // println!("{:?}", user);
        match result {
            None => Ok(None),
            Some(res) => Ok(Some(res)),
        }
    }
}
