use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

use super::skill_model::Skill;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct User {
    pub name: String,
    pub username: String,
    pub password: String,
    pub email: String,
    pub role: i32,
    pub field: String,                // 0..2 -> 0:Admin, 1:Teacher, 2:Guest
    pub completed_skills: Vec<Skill>, //unaccessible if role > 1
    pub auth_token: String,
    pub active: bool, //maybe add some other relevant data here
}
