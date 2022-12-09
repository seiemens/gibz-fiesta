#[cfg(test)]
pub mod test {

    use std::str::FromStr;

    use crate::data::mongo_connector::Connector;
    use crate::models::user_model::User;
    use mongodb::bson::oid::ObjectId;
    use mongodb::bson::Bson;

    use crate::rocket;

    async fn init_testing_env() -> Connector {
        Connector::init_test().await
    }

    #[tokio::test]
    async fn test_create_user() {
        // Set up the test environment

        let id = ObjectId::from_str(&"wad").unwrap();
        let mogge_user = User {
            _id: Some(id),
            name: Some("MOGGE".to_string()),
            username: "MOGGE".to_string(),
            password: "MOGGE".to_string(),
            email: Some("MOGGE@MOGGE.com".to_string()),
            role: Some(0),
            field: Some("MOGGE".to_string()),
            completed_skills: Some(Vec::new()),
            marked_skills: Some(Vec::new()),
            auth_token: Some("MOGGE".to_string()),
            active: Some(true),
        };

        let db = init_testing_env().await;

        // Call the API endpoint
        let response = db.create_user(mogge_user).await;
        //check if ID matches
        assert_ne!(response.unwrap().inserted_id, Bson::ObjectId(id));
    }
}
