use diesel::prelude::*;
use notes::*;
use std::env;
use self::models::{User, Note};

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
    use self::schema::users::dsl::*;

    if args.len() == 0 {
        return;
    }

    if args.contains(&"list".to_string()) {
        list_users(connection);

        if args.len() == 1 {
            return;
        }
    }

    match args[0].as_str() {
            "create" => match args[1].as_str() {
            "user" => {
                if !args[2].is_empty() {
                    let result : Vec<User> = users.select(User::as_select()).filter(name.eq(args[2].trim().to_string().clone())).load(connection).expect("could not load users from database");
                   if result.is_empty() {
                      create_user(connection, &args[2]); 
                   }
                   else {
                        println!("User with name \'{}\' already exists with ID: {}", result[0].name, result[0].id);
                   }
                }
            }
            _ => ()
        },
        "delete" => {
            match args[1].as_str() {
                "user" => if !args[2].is_empty() {
                        delete_user(connection, &args[2]);
                },
                _ => () 
            }
        },
        "edit" => match args[1].as_str() {
            "user" => if !args[2].trim().is_empty() && !args[3].trim().is_empty() {
                    let result : Vec<User> = users.select(User::as_select()).filter(name.eq(args[3].trim().to_string().clone())).load(connection).expect("could not load users from database");
                if result.is_empty() {
                    edit_user(connection, &args[2].trim().to_string(), &args[3].trim().to_string());
                }
                else {
                    println!("User with name \'{}\' already exists with ID: {}", result[0].name, result[0].id);
                }
            },
            _ => (),
        },
        "help" => {
            println!("notes <COMMAND> (-flags)");
            println!("[create user name || \'name\'] : creates a new user with name the provided name (case sensitive)");
            println!("[delete user name || \'name\'] : deletes the user with the selected name (case sensitive)");
            println!("[edit user previous_name || \'previous_name\' new_name || \'new_name\'] : edit the name of a user with the provided new name, if there isn't another user with same name");
            println!("[list] : list all users registered");
        },
        _ => (),
    }
}
