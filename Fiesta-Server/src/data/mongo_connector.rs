/*
----- SUMMARY -----
-> This is basically the Base Layer of the Backend.
*/

extern crate dotenv;

use crate::{
    helpers::{endecr, grandmas_bakery::biscuit},
    models::{skill_model::Skill, user_model::User},
};
use dotenv::dotenv;
use mongodb::{
    bson::doc,
    error::Error,
    options::{FindOneOptions, FindOptions},
    results::{DeleteResult, InsertOneResult, UpdateResult},
    Client, Collection, Cursor,
};
use rocket::http::{Cookie, CookieJar, Status};
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
----- AUTH CHECKER -----
*/
impl Connector {
    /// Verify the authenticity of a request.
    pub async fn verify_auth(&self, token: String) -> bool {
        //extract value from biscuit

        let filter = doc! {"auth_token":token};

        let result = self.user_col.find_one(filter, None).await;

        if let Ok(None) = result {
            return false;
        } else {
            return true;
        }
    }
}

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

    ///update password of specified user
    pub async fn update_user(
        &self,
        u: String,
        pw: String,
        auth: String,
    ) -> Result<UpdateResult, Error> {
        //check auth
        let filter = doc! { "username":u, "auth_token":auth };
        let update = doc! {"$set": {"password":pw}};
        let result = self.user_col.update_one(filter, update, None).await?;
        return Ok(result);
    }

    pub async fn delete_user(&self, u: User) -> Result<DeleteResult, Error> {
        let filter = doc! {"username":u.username, "password":u.password};
        let result = self.user_col.delete_one(filter, None).await?;
        return Ok(result);
    }
}
