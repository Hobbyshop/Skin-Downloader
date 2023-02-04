use std::{collections::HashMap};

use reqwest::Client;

pub async fn get_as_json(url: String) -> Result<HashMap<String, String>, reqwest::Error> {
    let response = Client::new()
        .get(url)
        .send()
        .await?
        .json::<HashMap<String, String>>()
        .await?;
    
    Ok(response)
}

pub async fn get_as_bytes(url: String) -> Result<Vec<u8>, reqwest::Error> {
    let response = Client::new()
        .get(url)
        .send()
        .await?
        .text()
        .await?;

    Ok(Vec::from(response.as_bytes()))
}
