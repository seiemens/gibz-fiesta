/*
----- SUMMARY -----
-> This is basically the Base Layer of the Backend.
*/

extern crate dotenv;

use std::env;
use dotenv::dotenv;
use mongodb::{
    bson::{doc, oid::ObjectId},
    Client,
    Collection,
    error::Error, results::{DeleteResult, InsertOneResult, UpdateResult},
};
use rocket::{futures::TryStreamExt, http::Status};

use crate::models::{skill_model::Skill, user_model::User};

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
        //change the var 'key' to change the uri (contained in .env file)
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

    /// verify that its an admin
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
        // 'oid switch' -> Generate an ObjectId if its empty.
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

    /// get user based on password & username | used for login (mainly).
    pub async fn get_user(&self, u: User) -> Result<Option<User>, Error> {
        let filter = doc! { "username": u.username, "password": u.password, "active":true };
        let result = self.user_col.find_one(filter, None).await?;
        match result {
            None => Ok(None),
            Some(res) => Ok(Some(res)),
        }
    }

    /// Update a user based on its ObjectId - Property
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

    /// [ADMIN] - Return a vector containing all users for management.
    pub async fn get_users(&self) -> Result<Vec<User>, Status> {
        let mut cursor = self.user_col.find(None, None).await.unwrap();

        let mut array: Vec<User> = Vec::new();

        while let Ok(Some(user)) = cursor.try_next().await {
            array.push(user);
        }

        return Ok(array);
    }

    /// Return a user based on params in the url.
    pub async fn get_user_profile(&self, username: String) -> Result<Option<User>, Error> {
        let filter = doc! { "username": username};
        let result = self.user_col.find_one(filter, None).await?;

        match result {
            None => Ok(None),
            Some(res) => Ok(Some(res)),
        }
    }
}

/*
----- SKILLS - RELATED FUNCTIONS -----
*/
impl Connector {
    /// create a skill based on properties
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

    /// [ADMIN] - Return a vector containing all skills.
    pub async fn get_skills(&self) -> Result<Vec<Skill>, Status> {
        let mut cursor = self.skill_col.find(None, None).await.unwrap();

        let mut array: Vec<Skill> = Vec::new();
        while let Ok(Some(user)) = cursor.try_next().await {
            // println!("{:?}", user);
            array.push(user);
        }

        return Ok(array);
    }

    /// [ADMIN] - Replace an existing Object with a new one, keeping the ObjectId
    pub async fn update_skill(&self, mut s: Skill) -> Result<InsertOneResult, Error> {
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
    /// Delete a Skill based on its ObjectId.
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
    /// Mark a skill, again using the ObjectId and Auth Token of the User -> Cannot be manipulated by admin.
    pub async fn mark_skill(&self, skill: ObjectId, auth: String) -> Result<UpdateResult, Error> {
        let filter = doc! {"auth_token":auth};
        let user = self.user_col.find_one(filter.clone(), None).await?;
        let skills_vec = user.unwrap().marked_skills.unwrap();

        // iterate over 'skills_vec' and check if it contains the skill
        if skills_vec
            .iter()
            .find(|f: &&ObjectId| f.to_string() == skill.to_string())
            .is_some()
        {
            // either remove it from 'marked_skills' if already present...
            let result = self
                .user_col
                .update_one(filter, doc! {"$pull":{"marked_skills":skill}}, None)
                .await?;
            return Ok(result);
        } else {
            // ...or add it if not.
            let result = self
                .user_col
                .update_one(filter, doc! {"$push":{"marked_skills":skill}}, None)
                .await?;
            return Ok(result);
        }
    }

    /// Complete a skill using the ObjectId and Auth Token of the User -> Cannot be manipulated by admin.
    pub async fn complete_skill(&self, skill: String, auth: String) -> Result<UpdateResult, Error> {
        let filter = doc! {"auth_token":auth};
        let user = self.user_col.find_one(filter.clone(), None).await?;
        let skills_vec = user.unwrap().completed_skills.unwrap();

        // same principle as with 'mark()', iterating over the 'completed_skills' array on the user object
        if skills_vec.iter().find(|f| f.to_string() == skill).is_some() {
            // and remove / add it if present / not present.
            let result = self
                .user_col
                .update_one(filter, doc! {"$pull":{"completed_skills":skill}}, None)
                .await?;
            return Ok(result);
        } else {
            let result = self
                .user_col
                .update_one(filter, doc! {"$push":{"completed_skills":skill}}, None)
                .await?;
            return Ok(result);
        }
    }
}
