// Features
// - $ tool print <recipe name or part of it> -> Print recipe
// - $ tool add -> opens wizard
// - $ tool import <file> -> import
// - $ tool edit <name> -> opens wizard to edit
// - $ tool remove <name> -> confirmation + remove
// - $ tool use <ingredient> -> all recipes with that ingredient(s)
// - $ tool tags -> print all the tags
// - $ tool list <tag> -> all recipes with that tag(s).
// Other options:
// "help"
// "help <command>"
// "-f <path to recipes file. Uses data/recipes.sql otherwise>

extern crate rusqlite;
use rusqlite::Connection;

use std::collections::HashMap;
use std::env;

mod printing;
use printing::print;

mod recipes;

fn help(_args: Vec<String>) {
    println!("Help message here");
}

fn main() {
    let mut commands = HashMap::new();
    commands.insert("print", print);

    let args: Vec<String> = env::args().collect();

    if args.len() <= 1 {
        help(args);
        return;
    }

    let connection = Connection::open_with_flags(
        "data/recipes.db",
        rusqlite::OpenFlags::SQLITE_OPEN_READ_WRITE,
    ).unwrap();

    match commands.get::<str>(&args[1]) {
        Some(executor) => executor(connection, args),
        None => panic!("Unrecognized command {}", args[1]),
    }
}
