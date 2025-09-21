use std::fs;
use reqwest::{header::AUTHORIZATION};
use serde_json::Value;
use serde::Deserialize;
use std::env;

extern crate dotenv;

use dotenv::dotenv;


const BASE_URL: &str = "https://api.esv.org/v3/passage/text/?q=";
const CONFIG_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/config.toml");

#[derive(Deserialize)]
struct APIConfig {
    esv_api_key: String,
}

impl APIConfig {
    fn new() -> Result<APIConfig, String> {
        if let Ok(key) = env::var("ESV_API_KEY") {
            Ok(APIConfig { esv_api_key: key })
        }
        else {
            let contents: String = fs::read_to_string(CONFIG_PATH)
                .map_err(|_| format!("Unable to read file `{}`", CONFIG_PATH))?;
            let config: APIConfig = toml::from_str(&contents)
                .map_err(|_| format!("Unable to load data from `{}`", CONFIG_PATH))?;
            Ok(config)
        }
    }
}

struct PassageConfig {
    passage_query: String,
    esv_api_key: String,
}

impl PassageConfig {
    fn build(args:&[String]) -> Result<PassageConfig, &'static str> {
        if args.len() < 2 {
            return Err("Please provide a verse or passage as a command line argument.");
        }
        let passage_query = args[1].trim();
        if passage_query.is_empty() {
            return Err("Passage argument cannot be empty.");
        }

        let api_config = APIConfig::new();
        let esv_api_key = api_config.unwrap().esv_api_key;

        Ok(
            PassageConfig {
                passage_query: passage_query.to_string(),
                esv_api_key: esv_api_key,
            }
        )
    }
}


async fn get_passage_text(passage_config: &PassageConfig) -> Result<Value, String> {

    let passage_url = format!("{}{}", BASE_URL, passage_config.passage_query);
    let client = reqwest::Client::new();
    let response = client
        .get(passage_url)
        .header(AUTHORIZATION, format!("Token {}", passage_config.esv_api_key))
        .send()
        .await
        .map_err(|_| "Failed to send request".to_string())?;

    let response_text = response.text().await.map_err(|_| "Failed to get text".to_string())?;
    let v: Value = serde_json::from_str(&response_text).map_err(|_| "Failed to parse JSON".to_string())?;

    Ok(v)
}


#[tokio::main]
async fn main(){
    dotenv().ok();
    let args: Vec<String> = env::args().collect();

    let config = PassageConfig::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        std::process::exit(1);
    });

    match get_passage_text(&config).await {
        Ok(text) => {
            if let Some(passages) = text["passages"].as_array() {
                for passage in passages {
                    if let Some(passage_str) = passage.as_str() {
                        print!("{}", passage_str);
                        print!("\n\n");
                    }
                }
            } else {
                eprintln!("No passages found in response.");
            }
        },
        Err(e) => eprintln!("Error: {}", e),
    }
}
