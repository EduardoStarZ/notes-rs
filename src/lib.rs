pub mod models;
pub mod schema;

use diesel::prelude::*;
use dotenvy::dotenv;
use self::models::{NewUser, User};
use std::env;


pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn create_post(conn: &mut SqliteConnection, id: &i32, name: &String) -> User {
    use crate::schema::users;

     let new_user = NewUser { id, name };

    diesel::insert_into(users::table)
        .values(&new_user)
        .returning(User::as_returning())
        .get_result(conn)
        .expect("msg")
}