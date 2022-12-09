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
    bson::{doc, oid::ObjectId, Bson},
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

    pub async fn init_test() -> Self {
        dotenv().ok();
        //change the var 'key' to change the uri (check your .env file)
        let uri = env::var("MONGOURI").expect("MONGOURI HAS TO BE SET");

        let client = Client::with_uri_str(uri).await.unwrap();
        let db = client.database("fiesta_test");
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
        let filter = doc! {"auth_token":token, "role":1};

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
    pub async fn create_user(&self, mut u: User) -> Result<InsertOneResult, Error> {
        if u._id == None {
            u._id = Some(ObjectId::new());
        }

        let new = User {
            _id: u._id,
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

    pub async fn update_user(&self, u: User) -> Result<InsertOneResult, Error> {
        let new = User {
            _id: u._id,
            username: u.username,
            name: u.name,
            password: u.password,
            email: u.email,
            role: u.role,
            field: u.field,
            completed_skills: u.completed_skills,
            marked_skills: u.marked_skills,
            auth_token: u.auth_token,
            active: u.active,
        };

        self.user_col
            .delete_one(doc! { "_id": u._id }, None)
            .await?;

        let res = self.user_col.insert_one(new, None).await.ok();

        return Ok(res.unwrap());
    }

    pub async fn delete_user(&self, u: User) -> Result<DeleteResult, Error> {
        let filter = doc! {"_id":u._id};
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

    pub async fn deactivate_user(&self, id: String) -> Result<UpdateResult, Error> {
        let filter = doc! { "_id":id };
        let user = self.user_col.find_one(filter.clone(), None).await?;
        let result;
        if user.unwrap().active == Some(true) {
            result = self
                .user_col
                .update_one(filter, doc! {"$set":{"active":false}}, None)
                .await?;
        } else {
            result = self
                .user_col
                .update_one(filter, doc! {"$set":{"active":true}}, None)
                .await?;
        }
        return Ok(result);
    }
}

/*
----- SKILLS - RELATED FUNCTIONS -----
*/
impl Connector {
    pub async fn create_skill(&self, mut s: Skill) -> Result<InsertOneResult, Error> {
        if s._id == None {
            s._id = Some(ObjectId::new());
        }
        let new = Skill {
            _id: s._id,
            display_id: s.display_id,
            name: s.name,
            levels: s.levels,
        };
        let skill = self
            .skill_col
            .insert_one(new, None)
            .await
            .ok()
            .expect("Error creating skill");
        Ok(skill)
    }

    pub async fn get_skills(&self) -> Result<Vec<Skill>, Status> {
        let mut cursor = self.skill_col.find(None, None).await.unwrap();

        let mut array: Vec<Skill> = Vec::new();
        while let Ok(Some(user)) = cursor.try_next().await {
            println!("{:?}", user);
            array.push(user);
        }

        return Ok(array);
    }

    pub async fn update_skill(&self, mut s: Skill, auth: String) -> Result<InsertOneResult, Error> {
        if s._id == None {
            s._id = Some(ObjectId::new());
        }
        let new = Skill {
            _id: s._id,
            display_id: s.display_id,
            name: s.name,
            levels: s.levels,
        };

        self.skill_col
            .delete_one(doc! { "_id": s._id }, None)
            .await?;

        let res = self.skill_col.insert_one(new, None).await;

        return Ok(res.unwrap());
    }

    pub async fn delete_skill(&self, s: Skill) -> Result<DeleteResult, Error> {
        let new = Skill {
            _id: s._id,
            display_id: s.display_id,
            name: s.name,
            levels: s.levels,
        };

        let filter = doc! {"_id":new._id};

        let result = self.skill_col.delete_one(filter, None).await?;
        return Ok(result);
    }

    pub async fn mark_skill(&self, skill: ObjectId, auth: String) -> Result<UpdateResult, Error> {
        let filter = doc! {"auth_token":auth};
        let user = self.user_col.find_one(filter.clone(), None).await?;
        let skills_vec = user.unwrap().marked_skills.unwrap();

        if skills_vec.iter().find(|f| f._id == Some(skill)).is_some() {
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

    pub async fn complete_skill(
        &self,
        skill: ObjectId,
        auth: String,
    ) -> Result<UpdateResult, Error> {
        let filter = doc! {"auth_token":auth};
        let user = self.user_col.find_one(filter.clone(), None).await?;
        let skills_vec = user.unwrap().completed_skills.unwrap();

        if skills_vec.iter().find(|f| f._id == Some(skill)).is_some() {
            let result = self
                .user_col
                .update_one(filter, doc! {"$push":{"completed_skills":skill}}, None)
                .await?;
            return Ok(result);
        } else {
            let result = self
                .user_col
                .update_one(filter, doc! {"$pull":{"completed_skills":skill}}, None)
                .await?;
            return Ok(result);
        }
    }
}
