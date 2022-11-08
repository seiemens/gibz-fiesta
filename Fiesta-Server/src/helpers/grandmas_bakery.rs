use rocket::http::Cookie;

/*
----- BISCUIT GENERATOR -----
-> We love cookie clicker, don't we?
*/

/// Generate a biscuit (cookie, but we're using british terms here) with the given arguments.
pub fn biscuit(name: String, value: String) -> Cookie<'static> {
    return Cookie::build(name, value).path("/").expires(None).finish(); // Setting the expiry date to 'None' sets it to expire when the session gets closed.
}
