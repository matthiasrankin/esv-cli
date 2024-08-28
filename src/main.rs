use std::fs;
use reqwest::{header::AUTHORIZATION};
use serde_json::Value;
use toml;
use serde::Deserialize;
use std::error::Error;

// TODO: Passage handling (multiple verses / single verses, multiple chapters / single chapter)

#[derive(Deserialize, Debug)]
struct Config {
    tokens: Tokens
}

#[derive(Deserialize, Debug)]
struct Tokens {
    esv_api: String,
}

// struct Chapters {
//     start_chapter: i32,
//     end_chapter: i32
// }

// struct Verses {
//     start_verse: i32,
//     end_verse: i32,
// }

// struct Passage {
//     book: String,
//     chapters: Chapters,
//     verses: Verses,
// }

// impl Passage {
//     fn from_str(passage_descriptor: &str) -> Passage {

//         let passage = Passage {
//             book: ,
//             chapter: ,
//             verse: 
//         }
//     }
// }


fn load_config(filename: &str) -> Result<Config, String> {
    let contents: String = fs::read_to_string(filename).map_err(|_| format!("Unable to read file `{}`", filename))?;
    let config: Config = toml::from_str(&contents).map_err(|_| format!("Unable to load data from `{}`", filename))?;
    Ok(config)
}


fn get_esv_api_key() -> Result<String, Box<dyn Error>> {
    let config = load_config("config.toml")?;
    Ok(config.tokens.esv_api)
}


async fn get_request() -> Value {

    let esv_api_key = get_esv_api_key();

    let client = reqwest::Client::new();
    let response = client
        .get("https://api.esv.org/v3/passage/text/?q=John+11-12")
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
    // let v = get_request().await;
    // println!("{}", v["passages"][0]);
    let passage_desc = "John 11:35-37";

    print!("{}\n", passage_desc);

    let split: Vec<&str> = passage_desc.split(" ").collect();
    let [book, start_end] = &split[..] else { return Err("Split") };

    let start_end = split.get(1).unwrap();
    let start_split: Vec<&str> = start_end.split("-").collect();
    let start = start_split.get(0).unwrap();

    print!("{}\n", book);
    print!("{}", start);

    // passage = Passage.from_str(&passage_desc);

}

// 11:35 - 12:4
// 11-12:4
// 11:35 - 12
// 11-12
// 11:35-36
// 11
// 11:35