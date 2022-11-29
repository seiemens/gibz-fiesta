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
use futures::stream::StreamExt;
use mongodb::{
    bson::doc,
    error::Error,
    options::{FindOneOptions, FindOptions},
    results::{DeleteResult, InsertOneResult, UpdateResult},
    Client, Collection, Cursor,
};
use rocket::{
    futures::{self, TryStreamExt},
    http::{Cookie, CookieJar, Status},
    serde::json::Json,
};
use std::{env, result, vec};
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
    pub async fn verify_auth(&self, token: String) -> Result<User, bool> {
        let filter = doc! {"auth_token":token};

        let result = self.user_col.find_one(filter, None).await;

        if let Ok(None) = result {
            return Err(false);
        } else {
            return Ok(result.unwrap().unwrap());
        }
    }

    //verify that its an admin
    pub async fn verify_admin(&self, token: String) -> Result<User, bool> {
        let filter = doc! {"auth_token":token, "role":0};

        let result = self.user_col.find_one(filter, None).await;

        if let Ok(None) = result {
            return Err(false);
        } else {
            return Ok(result.unwrap().unwrap());
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
            marked_skills: u.marked_skills,
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
    pub async fn update_user(&self, u: String, pw: String) -> Result<UpdateResult, Error> {
        let filter = doc! { "username":u };
        let update = doc! {"$set": {"password":pw}};
        let result = self.user_col.update_one(filter, update, None).await?;
        return Ok(result);
    }

    pub async fn delete_user(&self, u: String) -> Result<DeleteResult, Error> {
        let filter = doc! {"username":u};
        let result = self.user_col.delete_one(filter, None).await?;
        return Ok(result);
    }

    pub async fn get_users(&self) -> Result<Vec<User>, Status> {
        let mut cursor = self.user_col.find(None, None).await.unwrap();

        let mut array: Vec<User> = Vec::new();

        while let Ok(Some(user)) = cursor.try_next().await {
            array.push(user);
        }

        return Ok(array);
    }
}

/*
----- SKILLS - RELATED FUNCTIONS -----
*/
impl Connector {
    pub async fn create_skill(&self, s: Skill) -> Result<InsertOneResult, Error> {
        let new = Skill {
            name: s.name,
            recommended_group: s.recommended_group,
            subcategories: s.subcategories,
        };
        let skill = self
            .skill_col
            .insert_one(new, None)
            .await
            .ok()
            .expect("Error creating skill");
        Ok(skill)
    }

    pub async fn get_skills(&self) -> Result<Option<Skill>, Error> {
        let result = self.skill_col.find_one(None, None).await?;
        match result {
            None => Ok(None),
            Some(res) => Ok(Some(res)),
        }
    }

    pub async fn update_skill(&self, s: String, n: Skill) -> Result<UpdateResult, Error> {
        let filter = doc! { "_id":s };
        let update = doc! {"$set": {"name":n.name}};
        let result = self.skill_col.update_one(filter, update, None).await?;
        return Ok(result);
    }

    pub async fn delete_skill(&self, s: String) -> Result<DeleteResult, Error> {
        let filter = doc! {"_id":s};
        let result = self.user_col.delete_one(filter, None).await?;
        return Ok(result);
    }

    pub async fn mark_skill(&self, skill: String, auth: String) -> Result<UpdateResult, Error> {
        let filter = doc! {"auth_token":auth};
        let user = self.user_col.find_one(filter.clone(), None).await?;
        let skills_vec = user.unwrap().marked_skills.unwrap();

        // TODO: Change this to search the _id (ObjectID) instead of name
        if skills_vec.iter().any(|i| i.name.to_string() != skill) {
            let result = self
                .user_col
                .update_one(filter, doc! {"$push":{"marked_skills":skill}}, None)
                .await?;
            return Ok(result);
        } else {
            let result = self
                .user_col
                .update_one(filter, doc! {"$pull":{"marked_skills":skill}}, None)
                .await?;
            return Ok(result);
        }
    }

    // TODO: Implement "Add Resource to Skill"
    // TODO: Implement "Remove Resource from Skill"
    // TODO: Implement "Complete Skill"
}
