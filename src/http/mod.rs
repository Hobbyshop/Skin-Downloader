use std::{collections::HashMap};

use bytes::Bytes;
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

pub async fn get_as_bytes(url: String) -> Result<Bytes, reqwest::Error> {
    let response = Client::new()
        .get(url)
        .send()
        .await?
        .bytes()
        .await?;

    Ok(response)
}
