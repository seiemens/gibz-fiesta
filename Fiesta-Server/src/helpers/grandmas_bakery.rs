use std::time::Duration;
use rocket::http::{Cookie, CookieJar, SameSite};
use rocket::time::OffsetDateTime;

/*
----- BISCUIT GENERATOR -----
-> We love cookie clicker, don't we?
*/

/// Generate a biscuit (cookie, but we're using british terms here) with the given arguments.
pub fn biscuit(name: String, value: String) -> Cookie<'static> {
    return Cookie::build(name, value)
        .path("/")
        .expires(OffsetDateTime::now_utc()+Duration::from_secs(3600*24*2))
        .secure(true)
        .same_site(SameSite::None)
        .finish(); // Setting the expiry date to 'None' sets it to expire when the session gets closed.
}

/// Used to extract value from cookie.
pub fn get_biscuit_recipe(jar: &CookieJar<'_>, name: String) -> String {
    return String::from(jar.get(&name).map(|cookie| cookie.value()).unwrap());
}
