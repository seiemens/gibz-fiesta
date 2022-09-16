use rocket::serde::Deserialize;
#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct User<'a> {
    pub name: &'a str,
    pub email: &'a str,
    pub role: i32,              // 0..2 -> 0:Admin, 1:Teacher, 2:Guest
    pub skills: Vec<Skill<'a>>, //unaccessible if role > 1
    pub auth_token: &'a str,
    //maybe add some other relevant data here
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Skill<'a> {
    name: &'a str,
    skill_level: i32,
    recommended_group: &'a str, //[OPTIONAL] usage: to recommend certain skills / competences to certain teachers (e.g. IT Teachers)
}
