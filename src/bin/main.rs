use std::env;
use notes::*;
use processor::*;

fn main() {
    let start: u128 = counter();

    let connection = &mut establish_connection();

    process_arg(connection, get_args());

    let end: u128 = counter();

    println!("\n\nExecution time: {} ms", end - start);
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
