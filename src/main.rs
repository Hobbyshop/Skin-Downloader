pub mod logger;

use std::{io::{stdout, Write, stdin}, collections::HashMap};
use regex::Regex;
use reqwest::Client;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    println!("Please enter a valid username, its skin will be downloaded");

    let username = get_username_from_io().unwrap();

    let uuid = get_uuid_from_http(username).await.unwrap();
    println!("{}", uuid);

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

async fn get_uuid_from_http(username: String) -> Result<String, reqwest::Error> {
    let response = Client::new()
        .get("https://api.mojang.com/users/profiles/minecraft/".to_owned() + username.as_str())
        .send()
        .await?
        .json::<HashMap<String, String>>()
        .await?;
    
    Ok(response["id"].to_string())
}
