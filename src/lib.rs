pub mod models;
pub mod schema;

use diesel::prelude::*;
use dotenvy::dotenv;
use self::models::{User, Note};
use self::models::{NewUser, NewNote};
use std::env;
use rand::{thread_rng, Rng};
use self::schema::users::{self, dsl::*};
use self::schema::notes::{self, dsl::*};

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn list_users(connection : &mut SqliteConnection) {
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

fn user_uuid(connection : &mut SqliteConnection) -> u32 {
    let mut rng = rand::thread_rng();

    loop {
        let uuid = rng.gen::<u32>();

        let results : Vec<User> = users.select(User::as_select()).filter(users::id.eq(&(uuid as i32))).load(connection).expect("could not load users from database");
    
        if results.is_empty() {
            return uuid;
        }
    }
}

pub fn create_user(connection: &mut SqliteConnection, username: &String) {
    let new_user = NewUser { id : &(user_uuid(connection) as i32), name : &username.trim().to_string() };

    diesel::insert_into(users::table)
        .values(&new_user)
        .execute(connection).unwrap();
}

pub fn delete_user(connection : &mut SqliteConnection, username: &String) {
    diesel::delete((users::table).filter(users::name.eq(username))).execute(connection).unwrap();
}

pub fn edit_user(connection: &mut SqliteConnection, previous_name : &String , new_name: &String) {
    diesel::update(users::table).filter(users::name.eq(previous_name)).set(users::name.eq(new_name)).execute(connection).expect("could not load user from database");
}

fn note_uuid(connection : &mut SqliteConnection) -> u32 {
    let mut rng = thread_rng();
    loop {
        let uuid : u32 = rng.gen::<u32>();
        
        let results : Vec<Note> = notes.select(Note::as_select()).filter(notes::id.eq(uuid as i32)).load(connection).expect("could not load notes from database");

        if results.is_empty() {
            return uuid;
        }
    }
}

pub fn create_note(connection : &mut SqliteConnection, note_name : &String, text : &String, u_id : &i32) {
    let new_note = NewNote {id: &(note_uuid(connection) as i32), name : note_name, content: text, user_id: u_id };     

    diesel::insert_into(notes::table)
        .values(&new_note)
        .execute(connection)
        .expect("could not load notes from database");
}

pub fn delete_note(connection: &mut SqliteConnection, note_name : String) {
    diesel::delete(notes::table).filter(notes::name.eq(note_name)).execute(connection).unwrap();         
}

pub fn edit_note(connection: &mut SqliteConnection, old_name : String, new_name : String) {
}
