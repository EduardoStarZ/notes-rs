pub mod models;
pub mod noting;
pub mod schema;
pub mod profile;
pub mod processor;

use diesel::prelude::*;
use dotenvy::dotenv;
use std::{
    env,
    time::{SystemTime, UNIX_EPOCH},
};

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn counter() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis()
}
