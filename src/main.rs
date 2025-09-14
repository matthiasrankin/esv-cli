use std::fs;
use reqwest::{header::AUTHORIZATION};
use serde_json::Value;
use toml;
use serde::Deserialize;
use std::error::Error;

#[derive(Deserialize, Debug)]
struct Config {
    tokens: Tokens
}

#[derive(Deserialize, Debug)]
struct Tokens {
    esv_api: String,
}

fn load_config(filename: &str) -> Result<Config, String> {
    let contents: String = fs::read_to_string(filename).map_err(|_| format!("Unable to read file `{}`", filename))?;
    let config: Config = toml::from_str(&contents).map_err(|_| format!("Unable to load data from `{}`", filename))?;
    Ok(config)
}


fn get_esv_api_key() -> Result<String, Box<dyn Error>> {
    let config = load_config("config.toml")?;
    Ok(config.tokens.esv_api)
}


async fn get_request(passage_desc: &str) -> Value {

    let esv_api_key = get_esv_api_key();

    let passage_url = format!("https://api.esv.org/v3/passage/text/?q={}", passage_desc);
    let client = reqwest::Client::new();
    let response = client
        .get(passage_url)
        .header(AUTHORIZATION, format!("Token {}", esv_api_key.unwrap()))
        .send()
        .await;

    let response_text = match response {
        Ok(resp) => resp.text().await.unwrap_or_else(|_| "Failed to get text".to_string()),
        Err(_) => "Failed to send request".to_string(),
    };

    // Parse the string of data into serde_json::Value.
    let v: Value = serde_json::from_str(&response_text).expect("Failed to parse JSON");

    v
}


#[tokio::main]
async fn main(){
    let passage_desc = "Psalm 1";

    println!("{}", passage_desc);
    let text = get_request(passage_desc).await;
    println!("{}", text["passages"][0]);
}
