use diesel::prelude::*;
use notes::models::*;
use notes::*;
use std::env;

fn main() {
    // use self::schema::users::dsl::*;

    // let connection = &mut establish_connection();
    // let internal_handler = &mut establish_connection();

    // println!("Type the name of the user: ");
    // create_user(connection, &get_input());

    // list_users(connection);
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

fn process_arg(connection: &mut SqliteConnection, args: Vec<&str>) {
    match args[0] {
        "create" => match args[1] {
            "user" => {
                if !args[2].is_empty() {
                    create_user(connection, &args[2].to_string())
                }
            }
            _ => ()
        },
        "delete" => {}
        _ => (),
    }
}
