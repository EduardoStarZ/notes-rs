use diesel::prelude::*;
use notes::*;
use std::env;
use self::models::User;

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
                    create_user(connection, &args[2].to_string())
                }
            }
            _ => ()
        },
        "delete" => match args[1].as_str() {
            "user" => if !args[2].is_empty() {
                    let users_db : Vec<User> = users.select(User::as_select()).filter(name.eq(args[2].clone())).load(connection).expect("value does not exist at the user table in database");       
                    delete_user(connection, &args[2]);
                rearange_user_ids(connection, &(users_db[0].id as u32));
            
            },
            _ => () 
        }
        _ => (),
    }
}
