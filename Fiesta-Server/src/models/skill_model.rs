use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Skill {
    pub name: String,
    pub recommended_group: String, //[OPTIONAL] usage: to recommend certain skills / competences to certain teachers (e.g. IT Teachers)
    pub subcategories: Vec<SubSkill>,
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
    pub name: Option<String>,
    pub url: Option<String>,
}
