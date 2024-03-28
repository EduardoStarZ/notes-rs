use crate::establish_connection;
use crate::models::NewUser;
use crate::models::Profile;
use crate::schema::profile::{self, dsl::*};
use colored::Colorize;
use diesel::prelude::*;
use rand::Rng;

pub fn list_current_user() -> i32 {
    let connection: &mut SqliteConnection = &mut establish_connection();

    let users: Vec<Profile> = profile
        .select(Profile::as_select())
        .filter(profile::active.eq(true))
        .load(connection)
        .expect("could not load users from database");

    return users[0].id;
}

pub fn list_user(connection: &mut SqliteConnection) {
    let result: Vec<Profile> = profile
        .select(Profile::as_select())
        .load(connection)
        .expect("Error loading users from database");

    if result.is_empty() {
        println!("\n\nThere are no users registered");
        return;
    }

    println!(
        "\n\nShowing all users within the database, there are currently {} users\n\n\n",
        result.len()
    );

    for (i, x) in result.iter().enumerate() {
        let start_icon = format!(
            "{}{}{}",
            (i + 1).to_string().bright_black().on_bright_white(),
            ".".on_bright_white().bright_black(),
            "=========================".hidden().on_bright_white()
        );
        let end_icon = format!(
            "{}",
            "===========================".hidden().on_bright_white()
        );
        println!(
            "{start_icon}\nID: {}\nUser: {}\n{end_icon}\n",
            x.id.to_string().underline(),
            x.name.to_string().italic()
        );
    }
}

fn user_uuid(connection: &mut SqliteConnection) -> u32 {
    let mut rng = rand::thread_rng();

    loop {
        let uuid = rng.gen::<u32>();

        let results: Vec<Profile> = profile
            .select(Profile::as_select())
            .filter(profile::id.eq(&(uuid as i32)))
            .load(connection)
            .expect("could not load users from database");

        if results.is_empty() {
            return uuid;
        }
    }
}

pub fn create_user(connection: &mut SqliteConnection, username: &String) {
    let results: Vec<Profile> = profile
        .select(Profile::as_select())
        .load(connection)
        .expect("could not load users from database");

    
    let mut new_user : NewUser<'_> = NewUser {
        id: &(user_uuid(connection) as i32),
        name: &username.trim().to_string(),
        active: &false,
    };

    if results.is_empty() {
        new_user.active = &true;
    }

    diesel::insert_into(profile::table)
        .values(&new_user)
        .execute(connection)
        .unwrap();
}

pub fn delete_user(connection: &mut SqliteConnection, username: &String) {
    diesel::delete((profile::table).filter(profile::name.eq(username)))
        .execute(connection)
        .unwrap();
}

pub fn edit_user(connection: &mut SqliteConnection, previous_name: &String, new_name: &String) {
    diesel::update(profile::table)
        .filter(profile::name.eq(previous_name))
        .set(profile::name.eq(new_name))
        .execute(connection)
        .expect("could not load users from database");
}

pub fn activate_user(connection: &mut SqliteConnection, username: &String) {
    diesel::update(profile::table)
        .filter(profile::active.eq(true))
        .set(active.eq(false))
        .execute(connection)
        .expect("could not load user from database");

    diesel::update(profile::table)
        .filter(profile::name.eq(username))
        .set(active.eq(true))
        .execute(connection)
        .expect("could not load user from database");
}
