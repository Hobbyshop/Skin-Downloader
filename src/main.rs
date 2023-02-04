pub mod logger;
mod http;

use std::{io::{stdout, Write, stdin, Read}, fs::File};
use bytes::Bytes;
use regex::Regex;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    println!("Please enter a valid username, its skin will be downloaded");

    let username = get_username_from_io().unwrap();

    logger::log(&String::from(
        "Requesting user uuid from \"https://api.mojang.com/users/profiles/minecraft/".to_owned()
        + username.as_str()
        + "\""
    ));

    let user_data = http::get_as_json("https://api.mojang.com/users/profiles/minecraft/".to_owned() + username.as_str()).await
        .expect(&String::from("Could not find user with name \"".to_owned() + &username + "\""));
    
    let username = &user_data["name"];

    logger::log(&String::from(
        "Found user: \"".to_owned()
        + &username
        + "\" with uuid \""
        + user_data["id"].as_str()
        + "\""
    ));

    let skin_bytes = http::get_as_bytes("https://crafatar.com/skins/".to_owned() + &user_data["id"]).await?;

    logger::log(&String::from(
        "Downloaded skin from \"https://crafatar.com/skins/".to_owned()
        + &user_data["id"]
        + "\""
    ));

    let file = File::create(username.to_owned() + ".png").expect("Could not find nor create the image file!");
    write_to_file(file, skin_bytes);

    pause_cmd();

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

fn write_to_file(mut file: File, bytes: Bytes) {
    file.write(&bytes).unwrap();
    logger::log("Image saved");
}

fn pause_cmd() {
    logger::log(">> Press any key to exit <<");
    let _ = stdout().flush();
    let _ = stdin().read(&mut [0u8]).unwrap();
}
