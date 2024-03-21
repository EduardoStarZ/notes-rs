pub mod models;
pub mod schema;

use diesel::prelude::*;
use dotenvy::dotenv;
use self::models::User;

use self::models::NewUser;
use std::env;


pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn list_users(connection : &mut SqliteConnection) {
    use self::schema::users::dsl::*;
    let result : Vec<User> = users.select(User::as_select()).load(connection).expect("Error loading users from database");

    println!("Showing all users within the database, there are currently {} users", result.len());

    for x in result {
        println!("ID: {}\n-------------------\nUser: {}\n\n", x.id, x.name);
    }
}

pub fn create_user(connection: &mut SqliteConnection, username: &String) {
    use self::schema::users::{self, dsl::*};

    let u_id : &i32 = &(users.select(User::as_select()).load(connection).expect("Error loading users from database").len() as i32 + 1);

    let new_user = NewUser { id : u_id, name : username };

    diesel::insert_into(users::table)
        .values(&new_user)
        .execute(connection).unwrap();
}

pub fn delete_user(connection : &mut SqliteConnection, username: &String) {
    use self::schema::users::{self, dsl::*};

    diesel::delete((users::table).filter(name.eq(username))).execute(connection).unwrap();
}

pub fn rearange_user_ids(connection: &mut SqliteConnection, start : &u32) {
    use self::schema::users::{self, dsl::*};

    let size : i32 = users.select(User::as_select()).load(connection).expect("Error loading users from database").len() as i32;

    for x in *start..size as u32 + 1{
        diesel::update(users::table).filter(id.eq(x as i32)).set(id.eq(id-1)).execute(connection).unwrap();
    }
}
