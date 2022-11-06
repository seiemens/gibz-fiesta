use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

use super::skill_model::Skill;

#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    pub name: String,
    pub username: String,
    pub email: String,
    pub role: i32,                    // 0..2 -> 0:Admin, 1:Teacher, 2:Guest
    pub completed_skills: Vec<Skill>, //unaccessible if role > 1
    pub auth_token: String,
    pub password: String,
    pub active: bool, //maybe add some other relevant data here
}
