pub mod logger;

use std::io::{stdout, Write, stdin};

use regex::Regex;

fn main() {
    println!("Please enter a valid username, its skin will be downloaded");

    let username = get_username_from_io().unwrap();
    println!("{}", username);
}

fn get_username_from_io() -> Result<String, &'static str> {
    let mut input = String::new();
    print!("> ");

    let _ = stdout().flush();
    stdin().read_line(&mut input).expect("Invalid string input!");
    let input = String::from(input.trim());

    let ex = Regex::new(r"^[a-zA-Z-1-9_]{3,16}$").unwrap();

    if ex.is_match(&input) {
       Ok(input)
    } else {
        logger::error(format!("\"{}\" is not a valid username", input).as_str());
        Err("Invalid username format!")
    }
}
