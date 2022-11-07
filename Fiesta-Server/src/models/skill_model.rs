use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Skill {
    pub id: Option<ObjectId>,
    pub name: String,
    pub skill_level: i32,
    pub recommended_group: String, //[OPTIONAL] usage: to recommend certain skills / competences to certain teachers (e.g. IT Teachers)
}
