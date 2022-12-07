use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Skill {
    pub _id: Option<ObjectId>,
    pub display_id: Option<String>,
    pub name: String,
    pub levels: Option<Vec<SubSkill>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct SubSkill {
    pub id: i32,
    pub name: Option<String>,
    pub description: Option<String>,
    pub resources: Option<Resources>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Resources {
    pub id: i32,
    pub name: Option<String>,
    pub url: Option<String>,
}
