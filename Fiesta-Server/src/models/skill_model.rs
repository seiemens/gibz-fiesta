use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Skill {
    pub name: String,
    pub recommended_group: String, //[OPTIONAL] usage: to recommend certain skills / competences to certain teachers (e.g. IT Teachers)
    pub subcategories: Vec<SubSkill>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct SubSkill {
    pub name: Option<String>,
    pub description: Option<String>,
}
