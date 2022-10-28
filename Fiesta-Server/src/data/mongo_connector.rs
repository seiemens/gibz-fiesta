/*
----- SUMMARY -----
-> This is basically the Base Layer of the Backend. No real "logic" here, just inserting, editing and removing data
*/

extern crate dotenv;
extern crate r2d2;
extern crate r2d2_mongodb;
use dotenv::dotenv;
use r2d2::PooledConnection;
use r2d2_mongodb::{mongodb::db, ConnectionOptions, MongodbConnectionManager};
use std::{env, ops::Deref};

type Pool = r2d2::Pool<MongodbConnectionManager>;
pub struct Conn(pub PooledConnection<MongodbConnectionManager>);

pub fn init() -> Pool {
    /*
    ----- SETUP CONNECTION POOLS -----
    -> Opposed to express.js & co, connection pools have to be written "by hand", using r2d2.
    */

    // .env readouts
    dotenv().ok();
    let mongo_uri = env::var("MONGOURI").expect("MONGOURI MUST BE SET");
    let mongo_port = env::var("MONGOPORT").expect("MONGOPORT MUST BE SET");
    let db_name = env::var("DBNAME").expect("DBNAME MUST BE SET");
    let mongo_pw = env::var("MONGOPW").expect("MONGOPW MUST BE SET");
    let mongo_user = env::var("MONGOUSER").expect("MONGOUSER MUST BE SET");

    //configure the connection pool
    let manager = MongodbConnectionManager::new(ConnectionOptions {
        host: mongo_uri,
        port: mongo_port.parse::<u16>().unwrap(),
        db: db_name,
        username: Some(mongo_user),
        password: Some(mongo_pw),
    });

    match Pool::builder().max_size(64).build(manager) {
        Ok(pool) => pool,
        Err(e) => panic!("Error: failed to create pool {}", e),
    }
}

/*
    Close connection if its no longer used
*/
impl Deref for Conn {
    type Target = PooledConnection<MongodbConnectionManager>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
