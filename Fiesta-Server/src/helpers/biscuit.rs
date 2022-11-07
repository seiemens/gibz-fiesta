use rocket::http::{Cookie, CookieJar};

/*
----- BISCUIT GENERATOR -----
*/

/// Generate a biscuit (cookie, but we're using british terms here) with the given arguments.
pub fn biscuit(n: String, v: String) -> Cookie<'static> {
    return Cookie::build(n, v).path("/").expires(None).finish();
}
