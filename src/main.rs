pub mod logger;
mod http;

use std::{io::{stdout, Write, stdin}, fs::File};
use regex::Regex;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    println!("Please enter a valid username, its skin will be downloaded");

    let username = get_username_from_io().unwrap();

    let user_data = http::get_as_json("https://api.mojang.com/users/profiles/minecraft/".to_owned() + username.as_str()).await.unwrap();
    let username = &user_data["name"];

    let skin_image = http::get_as_bytes("https://crafatar.com/skins/6d53bf220ad14e6b892a4cb6032e8a9f".to_string()).await?;

    let file = File::create(username.to_owned() + ".png").expect("Could not find nor create the image file!");
    write_to_file(file, skin_image);

    Ok(())
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

fn write_to_file(mut file: File, bytes: Vec<u8>) {
    let byte_slice = bytes.as_slice();
    file.write_all(byte_slice).expect("Could not write bytes to file");
}
