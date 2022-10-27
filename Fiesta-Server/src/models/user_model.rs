use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    pub id: Option<ObjectId>,
    pub name: String,
    pub email: String,
    pub role: i32,                    // 0..2 -> 0:Admin, 1:Teacher, 2:Guest
    pub completed_skills: Vec<Skill>, //unaccessible if role > 1
    pub auth_token: String,
    //maybe add some other relevant data here
}
