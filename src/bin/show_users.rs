use notes::models::*;
use diesel::prelude::*;
use notes::*;
use std::io;

fn main() {
    let mut input : String = String::new();

    io::stdin().read_line(&mut input).expect("could not read line");


}

fn insert_user() {

}

fn list_users() {
    use self::schema::users::dsl::*;

    let connection : &mut SqliteConnection = &mut establish_connection();
    let result : Vec<User> = users.select(User::as_select()).load(connection).expect("Error loading users from database");

    println!("Showing all users within the database, there are currently {} users", result.len());

    for x in result {
        println!("ID: {}\n-------------------\nUser: {}\n\n", x.id, x.name);
    }
}