pub mod models;
pub mod schema;
use self::models::{NewNote, NewUser};

use self::models::{Note, User};
use self::schema::note::{self, dsl::*};
use self::schema::user::{self, dsl::*};
use diesel::prelude::*;
use dotenvy::dotenv;
use rand::{thread_rng, Rng};
use std::env;

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn list_user(connection: &mut SqliteConnection) {
    let result: Vec<User> = user
        .select(User::as_select())
        .load(connection)
        .expect("Error loading users from database");

    if result.is_empty() {
        println!("\n\nThere are no users registered");
        return;
    }

    println!(
        "\n\nShowing all users within the database, there are currently {} users\n",
        result.len()
    );

    for x in result {
        println!(
            "===========================\nID: {}\nUser: {}\n===========================\n",
            x.id, x.name
        );
    }
}

fn user_uuid(connection: &mut SqliteConnection) -> u32 {
    let mut rng = rand::thread_rng();

    loop {
        let uuid = rng.gen::<u32>();

        let results: Vec<User> = user
            .select(User::as_select())
            .filter(user::id.eq(&(uuid as i32)))
            .load(connection)
            .expect("could not load users from database");

        if results.is_empty() {
            return uuid;
        }
    }
}

pub fn create_user(connection: &mut SqliteConnection, username: &String) {
    let new_user = NewUser {
        id: &(user_uuid(connection) as i32),
        name: &username.trim().to_string(),
        active: &false,
    };

    diesel::insert_into(user::table)
        .values(&new_user)
        .execute(connection)
        .unwrap();
}

pub fn delete_user(connection: &mut SqliteConnection, username: &String) {
    diesel::delete((user::table).filter(user::name.eq(username)))
        .execute(connection)
        .unwrap();
}

pub fn edit_user(connection: &mut SqliteConnection, previous_name: &String, new_name: &String) {
    diesel::update(user::table)
        .filter(user::name.eq(previous_name))
        .set(user::name.eq(new_name))
        .execute(connection)
        .expect("could not load users from database");
}

pub fn activate_user(connection: &mut SqliteConnection, username: &String) {
    diesel::update(user::table)
        .filter(user::active.eq(true))
        .set(active.eq(false))
        .execute(connection)
        .expect("could not load user from database");

    diesel::update(user::table)
        .filter(user::name.eq(username))
        .set(active.eq(true))
        .execute(connection)
        .expect("could not load user from database");
}

pub fn note_uuid(connection: &mut SqliteConnection) -> u32 {
    let mut rng = thread_rng();
    loop {
        let uuid: u32 = rng.gen::<u32>();

        let results: Vec<Note> = note
            .select(Note::as_select())
            .filter(note::id.eq(uuid as i32))
            .load(connection)
            .expect("could not load notes from database");

        if results.is_empty() {
            return uuid;
        }
    }
}

pub fn create_note(connection: &mut SqliteConnection, note_name: &String, text: &String) {
    let results: Vec<User> = user
        .select(User::as_select())
        .filter(user::active.eq(true))
        .load(connection)
        .expect("could not load users from database");

    let new_note = NewNote {
        id: &(note_uuid(connection) as i32),
        name: note_name,
        content: text,
        user_id: &results[0].id,
    };

    diesel::insert_into(note::table)
        .values(&new_note)
        .execute(connection)
        .expect("could not load notes from database");
}

pub fn delete_note(connection: &mut SqliteConnection, note_name: &String) {
    diesel::delete(note::table)
        .filter(note::name.eq(note_name))
        .execute(connection)
        .unwrap();
}

pub fn edit_note(connection: &mut SqliteConnection, old_name: &String, new_name: &String) {
    diesel::update(note::table)
        .filter(note::name.eq(old_name))
        .set(note::name.eq(new_name))
        .execute(connection)
        .expect("could not load notes from database");
}

pub fn list_note(connection: &mut SqliteConnection) {
    let current_user: Vec<User> = user
        .select(User::as_select())
        .filter(user::active.eq(true))
        .load(connection)
        .expect("could not load users from database");

    let result: Vec<Note> = note
        .select(Note::as_select())
        .filter(note::user_id.eq(current_user[0].id))
        .load(connection)
        .expect("Error loading users from database");

    if result.is_empty() {
        println!("There are no notes registered");
        return;
    }

    println!(
        "\n\nShowing all notes within the database for user {}, there are currently {} notes \n",
        current_user[0].name,
        result.len()
    );

    for x in result {
        println!("\n===========================\nID: {}\nName: {}\nContent: \n\n\" {} \"\n\n===========================", x.id, x.name, x.content);
    }
}
