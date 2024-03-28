use crate::models::NewProfile;
use crate::models::{Note, Profile};
use crate::schema::note::{self, dsl::*};
use crate::schema::profile::{self, dsl::*};
use colored::Colorize;
use diesel::prelude::*;
use rand::Rng;

pub fn note_uuid(connection: &mut SqliteConnection) -> u32 {
    let mut rng = rand::thread_rng();
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
    let results: Vec<Profile> = profile
        .select(Profile::as_select())
        .filter(profile::active.eq(true))
        .load(connection)
        .expect("could not load users from database");

    let new_note = NewProfile {
        id: &(note_uuid(connection) as i32),
        name: note_name,
        content: text,
        profile_id: &results[0].id,
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

pub fn edit_note_name(connection: &mut SqliteConnection, old_name: &String, new_name: &String) {
    diesel::update(note::table)
        .filter(note::name.eq(old_name))
        .set(note::name.eq(new_name))
        .execute(connection)
        .expect("could not load notes from database");
}

pub fn edit_note_content(
    connection: &mut SqliteConnection,
    note_name: &String,
    new_content: &String,
) {
    diesel::update(note::table)
        .filter(note::name.eq(note_name))
        .set(note::content.eq(new_content))
        .execute(connection)
        .expect("could not load notes from database");
}

pub fn list_note(connection: &mut SqliteConnection) {
    let current_user: Vec<Profile> = profile
        .select(Profile::as_select())
        .filter(profile::active.eq(true))
        .load(connection)
        .expect("could not load users from database");

    if current_user.is_empty() {
        println!("\nThere is no active user selected");
        return;
    }

    let result: Vec<Note> = note
        .select(Note::as_select())
        .filter(note::profile_id.eq(current_user[0].id))
        .load(connection)
        .expect("Error loading users from database");

    if result.is_empty() {
        println!("There are no notes registered");
        return;
    }

    let id_icon = "ID".bold();
    let name_icon = "Name".bold();
    let content_icon = "Content".bold();

    println!(
        "\n\nShowing all notes within the database for user {}, there are currently {} notes \n\n\n",
        current_user[0].name.underline(),
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
        println!("{start_icon}\n{id_icon}: {}\n{name_icon}: {}\n{content_icon}: \n\n\t {} \n\n{end_icon}\n\n", x.id.to_string().underline(), x.name.italic(), x.content.on_bright_black());
    }
}
