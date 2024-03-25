pub mod models;
pub mod schema;

use diesel::prelude::*;
use dotenvy::dotenv;
use self::models::User;
use self::models::Note;

use self::models::NewUser;
use std::env;
use rand::Rng;

fn generate_UUID() -> i32 {
    use self::schema::{users::{self, dsl::*}, note}
}

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn list_users(connection : &mut SqliteConnection) {
    use self::schema::users::dsl::*;
    let result : Vec<User> = users.select(User::as_select()).load(connection).expect("Error loading users from database");

    if result.is_empty() {
        println!("There are no users registered");
        return;
    }

    println!("Showing all users within the database, there are currently {} users", result.len());

    for x in result {
        println!("ID: {}\n-------------------\nUser: {}\n\n", x.id, x.name);
    }
}

pub fn create_user(connection: &mut SqliteConnection, username: &String) {
    use self::schema::users;

    let mut rng = rand::thread_rng();
    
    let uuid = rng.gen::<u32>();

    let new_user = NewUser { id : &(uuid as i32), name : &username.trim().to_string() };

    diesel::insert_into(users::table)
        .values(&new_user)
        .execute(connection).unwrap();
}

pub fn delete_user(connection : &mut SqliteConnection, username: &String) {
    use self::schema::users::{self, dsl::*};

    diesel::delete((users::table).filter(name.eq(username))).execute(connection).unwrap();
}

pub fn edit_user(connection: &mut SqliteConnection, previous_name : &String , new_name: &String) {
    use self::schema::users::{self, dsl::*};

    diesel::update(users::table).filter(name.eq(previous_name)).set(name.eq(new_name)).execute(connection).expect("could not load user from database");
}

pub fn create_note(connection : SqliteConnection, name : &String, content : &String) {

}
