use tokio;
use std::{
    env,
    fs
};

use reqwest::StatusCode;

async fn get_code(url: &str) -> Result<StatusCode, &'static str> {
    let response = reqwest::get(url).await;
    match response {
        Ok(response) => {
            if response.status().is_success() {
                return Ok(response.status());
            }
        }
        Err(_) => {}
    }
    Err("Request failed")
}

#[tokio::main]
async fn main() -> Result<(), bool> {
    let var = get_code("https://users.rust-lang.org").await;
    match var {
        Ok(status) => println!("{}", status),
        Err(error) => println!("Error: {}", error),
    }
    Ok(())
}
