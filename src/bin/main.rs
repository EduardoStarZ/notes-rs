use self::models::{Note, User};
use self::schema::note::{self, dsl::*};
use self::schema::user::{self, dsl::*};
use colored::Colorize;
use diesel::prelude::*;
use notes::*;
use std::env;

fn main() {
    let connection = &mut establish_connection();

    process_arg(connection, get_args());
}

fn get_args() -> Vec<String> {
    let mut args: Vec<String> = Vec::new();
    for (i, arg) in env::args().enumerate() {
        if i != 0 {
            args.push(arg);
        }
    }
    return args;
}

fn process_arg(connection: &mut SqliteConnection, args: Vec<String>) {
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
                    let result: Vec<User> = user
                        .select(User::as_select())
                        .filter(user::name.eq(args[2].trim().to_string().clone()))
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
                    let result: Vec<User> = user
                        .select(User::as_select())
                        .filter(user::name.eq(args[3].trim().to_string().clone()))
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
                            .filter(note::user_id.eq(list_current_user()))
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
                            .filter(note::user_id.eq(list_current_user()))
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
            println!("notes <COMMAND> <SUBCOMMAND> (-flags)");
            println!("[create user name || \'name\'] : creates a new user with name the provided name (case sensitive)");
            println!("[delete user name || \'name\'] : deletes the user with the selected name (case sensitive)");
            println!("[edit user previous_name || \'previous_name\' new_name || \'new_name\'] : edit the name of a user with the provided new name, if there isn't another user with same name");
            println!("[edit note previous_name || \'previous_name\' new_name || \'new_name\'] : edit the name of a note with the provided new name, if there isn't another note with same name");
            println!("[list user | note] : list all users/notes registered");
        }
        _ => println!("{}", "some text as bold".bold()),
    }
}
