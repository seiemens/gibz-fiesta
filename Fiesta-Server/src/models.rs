pub struct User<'a> {
    name: &'a str,
    email: &'a str,
    access_type: i32,       // 0..2 -> 0:Admin, 1:Teacher, 2:Guest
    skills: Vec<Skill<'a>>, //unaccessible if access_type > 1
    auth_token: &'a str,
    //maybe add some other relevant data here
}

pub struct Skill<'a> {
    name: &'a str,
    skill_level: i32,
    recommended_group: &'a str, //[OPTIONAL] usage: to recommend certain skills / competences to certain teachers (e.g. IT Teachers)
}
