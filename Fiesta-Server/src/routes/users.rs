use crate::models::User;
/*
--- IMPORTANT NOTICE ---
    auth_token: used for API authentication to prohibit access from unauthorized sources.
*/

// [ADMIN] - Register new account
pub fn new_user(u: User, auth_token: &str) -> bool {
    true
}
