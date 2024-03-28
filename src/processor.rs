use crate::{
    models::{Note, Profile},
    schema::{
        note::{self, dsl::*},
        profile::{self, dsl::*},
    },
};
use crate::{noting::*, profile::*};
use colored::Colorize;
use diesel::prelude::*;

pub fn process_arg(connection: &mut SqliteConnection, args: Vec<String>) {
    if args.len() == 0 {
        return;
    }

    match args[0].as_str() {
        "list" => match args[1].as_str() {
            "user" | "users" => list_user(connection),
            "note" | "notes" => list_note(connection),
            _ => (),
        },
        "activate" => {
            if args[1] == "user" {
                activate_user(connection, &args[2]);
            }
        }
        "create" => match args[1].as_str() {
            "user" => {
                if !args[2].is_empty() {
                    let result: Vec<Profile> = profile
                        .select(Profile::as_select())
                        .filter(profile::name.eq(args[2].trim().to_string().clone()))
                        .load(connection)
                        .expect("could not load users from database");
                    if result.is_empty() {
                        create_user(connection, &args[2]);
                    } else {
                        println!(
                            "User with name \'{}\' already exists with ID: {}",
                            result[0].name, result[0].id
                        );
                    }
                }
            }
            "note" => {
                if !args[2].is_empty() {
                    let result: Vec<Note> = note
                        .select(Note::as_select())
                        .filter(note::name.eq(args[2].trim().to_string().clone()))
                        .load(connection)
                        .expect("could not load notes from database");

                    if result.is_empty() {
                        create_note(connection, &args[2], &args[3]);
                    } else {
                        println!(
                            "Note with name \'{}\' already exists with ID: {}",
                            result[0].name, result[0].id
                        );
                    }
                }
            }
            _ => (),
        },
        "delete" => match args[1].as_str() {
            "user" => {
                if !args[2].is_empty() {
                    delete_user(connection, &args[2]);
                }
            }
            "note" => {
                if !args[2].is_empty() {
                    delete_note(connection, &args[2]);
                }
            }
            _ => (),
        },
        "edit" => match args[1].as_str() {
            "user" => {
                if !args[2].trim().is_empty() && !args[3].trim().is_empty() {
                    let result: Vec<Profile> = profile
                        .select(Profile::as_select())
                        .filter(profile::name.eq(args[3].trim().to_string().clone()))
                        .load(connection)
                        .expect("could not load users from database");
                    if result.is_empty() {
                        edit_user(
                            connection,
                            &args[2].trim().to_string(),
                            &args[3].trim().to_string(),
                        );
                    } else {
                        println!(
                            "User with name \'{}\' already exists with ID: {}",
                            result[0].name, result[0].id
                        );
                    }
                }
            }
            "note" => match args[2].trim() {
                "name" => {
                    if !args[3].trim().is_empty() {
                        let result: Vec<Note> = note
                            .select(Note::as_select())
                            .filter(note::name.eq(args[3].trim().to_string().clone()))
                            .filter(note::profile_id.eq(list_current_user()))
                            .load(connection)
                            .expect("could not load users from database");

                        if !result.is_empty() {
                            edit_note_name(connection, &args[3], &args[4]);
                        } else {
                            println!("No note was found with name {}", args[3]);
                        }
                    }
                }
                "text" => {
                    if !args[3].trim().is_empty() {
                        let result: Vec<Note> = note
                            .select(Note::as_select())
                            .filter(note::name.eq(args[3].trim().to_string().clone()))
                            .filter(note::profile_id.eq(list_current_user()))
                            .load(connection)
                            .expect("could not load users from database");

                        if !result.is_empty() {
                            edit_note_content(connection, &args[3], &args[4]);
                        } else {
                            println!("No note was found with name {}", args[3])
                        }
                    }
                }
                _ => (),
            },
            _ => (),
        },
        "help" => {
            println!(
                "{} {} {} {}\n",
                "notes".bold(),
                "<COMMAND>".bright_red(),
                "<SUBCOMMANDS>".bright_cyan(),
                "--flags".bold()
            );
            println!("{} :", "CREATE".underline().bright_green());
            println!(
                "\t~> {} => {}",
                "user |<NAME> or \"<NAME>\"| ".on_black(),
                "creates a new user".italic().underline()
            );
            println!(
                "\t~> {} => {}",
                "note |<NAME> or \"<NAME>\"| & |<CONTENT> or \"<CONTENT>\"| ".on_black(),
                "creates a new note".italic().underline()
            );
            println!("{} :", "DELETE".underline().bright_red());
            println!(
                "\t~> {} => {}",
                "user |<NAME> or \"<NAME>\"| ".on_black(),
                "deletes the user".italic().underline()
            );
            println!(
                "\t~> {} => {}",
                "note |<NAME> or \"<NAME>\"| ".on_black(),
                "deletes the note".italic().underline()
            );
            println!("{} :", "EDIT".underline().bright_blue());
            println!(
                "\t~> {} => {}",
                "user |<OLD NAME> or \"<OLD NAME>\"| & |<NEW NAME> or \"<NEW NAME>\"| ".on_black(),
                "edits an user name".italic().underline()
            );
            println!(
                "\t~> {} => {}",
                "note name |<NAME> or \"<NAME>\"| ".on_black(),
                "edits a note name".italic().underline()
            );
            println!(
                "\t~> {} => {}",
                "note text |<NAME> or \"<NAME>\"| ".on_black(),
                "edits a note content".italic().underline()
            );
            println!("{} :", "LIST".underline().bright_yellow());
            println!(
                "\t~> {} => {}",
                "user".on_black(),
                "lists all users".italic().underline()
            );
            println!(
                "\t~> {} => {}",
                "notes".on_black(),
                "lists all notes".italic().underline()
            );
        }
        _ => (),
    }
}
