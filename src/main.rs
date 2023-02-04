use std::io::{stdout, Write, stdin};

use regex::Regex;

fn main() {
    println!("Please enter a valid username, its skin will be downloaded");

    let mut input = String::new();
    print!("> ");

    let _ = stdout().flush();
    stdin().read_line(&mut input).expect("Invalid string input!");

    let ex = Regex::new(r"^[a-zA-Z0-9_]{3,16}$").unwrap();
    println!("{}", ex.is_match(&input));
}
