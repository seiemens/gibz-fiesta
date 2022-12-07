use rand::{distributions::Alphanumeric, Rng};

/*
----- Generate a Random Token (String) with a specified length
*/
pub fn generate(len: i32) -> String {
    return rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(len as usize)
        .map(char::from)
        .collect();
}
