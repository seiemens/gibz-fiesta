#[cfg(test)]
mod tests {
use std::{thread, time, env};
use crate::rocket;
use rocket::local::asynchronous::Client;
use rocket::http::{Status, Cookie};
use rocket::serde::json::json;
use dotenv::dotenv;

    #[tokio::test]
    async fn test_create_user() {
        dotenv().ok();
        //change the var 'key' to change the uri (check your .env file)
        let token = env::var("TESTAUTHTOKEN").expect("TESTAUTHTOKEN HAS TO BE SET TO USE UNIT TESTS");

        let client = Client::tracked(rocket().await).await.expect("valid rocket instance");     
        let user_data = json!({
            "_id":{"$oid":"746573747465737474657374"},
            "name": "Test User",
            "username": "testuser",
            "password": "password",
            "email": "test@example.com",
            "role": 1,
            "field": "IT",
            "auth_token":"TOKENTOKENTOKENTOKEN",
            "active": true,
        });
        let response = client
            .post("/user/create")
            .body(user_data.to_string())
            .cookie(Cookie::new("auth_biscuit", token))
            .dispatch();
            assert_eq!(response.await.status(), Status::Ok);
    }

    #[tokio::test]
    async fn test_login_user(){

        let timeout = time::Duration::from_secs(2);
        thread::sleep(timeout);

        let client = Client::tracked(rocket().await).await.expect("valid rocket instance");     
        let user_data = json!({
            "username": "testuser",
            "password": "password"
        });

        let response = client
        .post("/user/login")
        .cookie(Cookie::new("auth_biscuit", "TOKENTOKENTOKENTOKEN"))
        .body(user_data.to_string())
        .dispatch();
        assert_eq!(response.await.status(), Status::Ok);
    }
    #[tokio::test]
    async fn test_logout_user(){

        let ten_millis = time::Duration::from_secs(1);
        thread::sleep(ten_millis);

        let client = Client::tracked(rocket().await).await.expect("valid rocket instance");     

        let response = client
        .get("/user/logout")
        .dispatch();
        assert_eq!(response.await.status(), Status::Ok);
    }

    #[tokio::test]
    async fn test_verify_auth() {

        let timeout = time::Duration::from_secs(1);    
        thread::sleep(timeout);

        let client = Client::tracked(rocket().await).await.expect("valid rocket instance");
        let response = client
        .get("/user/auth")
        .cookie(Cookie::new("auth_biscuit", "TOKENTOKENTOKENTOKEN"))
            .dispatch();

        assert_eq!(response.await.status(), Status::Ok);
    }

    #[tokio::test]
    async fn test_update_user() {

        let timeout = time::Duration::from_secs(1);    
        thread::sleep(timeout);

        let client = Client::tracked(rocket().await).await.expect("valid rocket instance");     
        let user_data = json!({
            "_id":{"$oid":"746573747465737474657374"},
            "name": "Test User",
            "username": "testuser",
            "password": "password",
            "email": "test@example.com",
            "role": 1,
            "field": "KV",
            "auth_token":"TOKENTOKENTOKENTOKEN",
            "active": true,
        });
        let response = client
            .post("/user/update")
            .cookie(Cookie::new("auth_biscuit", "TOKENTOKENTOKENTOKEN"))
            .body(user_data.to_string())
            .dispatch();

        assert_eq!(response.await.status(), Status::Ok);
    }

    #[tokio::test]
    async fn test_get_all_users() {

        let timeout = time::Duration::from_secs(1);    
        thread::sleep(timeout);

        let client = Client::tracked(rocket().await).await.expect("valid rocket instance");
        let response = client
        .get("/user/all")
        .cookie(Cookie::new("auth_biscuit", "TOKENTOKENTOKENTOKEN"))
            .dispatch();

        assert_eq!(response.await.status(), Status::Ok);
    }

    #[tokio::test]
    async fn test_delete_user() {

        let timeout = time::Duration::from_secs(2);    
        thread::sleep(timeout);

        let client = Client::tracked(rocket().await).await.expect("valid rocket instance");

        let user_data = json!({
            "_id":{"$oid":"746573747465737474657374"},
            "username": "",
            "password": "",
        });

        let response = client
        .post("/user/delete")
        .cookie(Cookie::new("auth_biscuit", "TOKENTOKENTOKENTOKEN"))
        .body(user_data.to_string())
            .dispatch();

        assert_eq!(response.await.status(), Status::Accepted);
    }
    
}
