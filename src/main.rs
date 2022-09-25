use tokio;
use reqwest;
use serde_json;
use std::env;
use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref APP_ID: String = env::var("APP_ID").unwrap();
    pub static ref GUILD_ID: String = env::var("GUILD_ID").unwrap();
    pub static ref DISCORD_TOKEN: String = env::var("DISCORD_TOKEN").unwrap();
    pub static ref PUBLIC_KEY: String = env::var("PUBLIC_KEY").unwrap();
}

// No fields with default values are used
const EXAMPLE_COMMAND: &str = r#"{
    "content": "Hi! I'm Judy-2D. I'm a hooty-tooty disco cutie!",
    "tts": false,
    "embeds": [{
      "title": "More Grease!",
      "description": "hooty-tooty disco cutie! hooty-tooty disco cutie! hooty-tooty disco cutie!"
    }]
  }
  "#;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    println!("Application ID: {}", *APP_ID);
    
    let url: String = format!("https://discord.com/api/v9/channels/{}/messages", "979894864709824582");

    let res = reqwest::Client::new()
        .post(url)
        .body(EXAMPLE_COMMAND)
        .header("Authorization", format!("Bot {}", *DISCORD_TOKEN)) 
        .header("Content-Type", "application/json") 
        .send()
        .await?
        .text()
        .await?;
    
    let result = serde_json::from_str::<HashMap<String, serde_json::Value>>(&res);
    
    println!("{:?}", result);
    
    Ok(())
}
