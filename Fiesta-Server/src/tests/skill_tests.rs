#[cfg(test)]
mod tests {
use crate::rocket;
use rocket::local::asynchronous::Client;
use rocket::http::{Status};

    #[tokio::test]
    async fn test_get_all_skills() {
        let client = Client::tracked(rocket().await).await.expect("valid rocket instance");
        let response = client
            .get("/skill/all")
            .dispatch();

        assert_eq!(response.await.status(), Status::Ok);
    }
}
