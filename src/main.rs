use tokio;
use clap::Parser;
use clap;
use std::fs;
use std::io::Read;
use reqwest::{StatusCode, get};

#[derive(Parser, Debug)]
#[clap(author="rustbuster", version="0.0.1", about="Fust and async url buster written on Rust.")]
struct Cli{
    #[clap(short='w', long="wordlist")]
    /// wordlist path to bust
    // pattern: String, 
    path: std::path::PathBuf,

    #[clap(short='u', long="url")]
    /// bust url
    url: String,
}

async fn get_code(url: &str) -> Result<StatusCode, &'static str> {
    let response = reqwest::get(url).await;
    match response {
        Ok(response) => return Ok(response.status()),
        Err(_) => {}
    }
    Err("Request failed")
}

#[tokio::main]
async fn main() -> Result<(), bool> {
    // println!("{}", get_code("https://monkeytype.com/").await.unwrap());


    let mut output: Vec<String> = Vec::new();

    let args = Cli::parse();

    let mut file =  fs::File::open(args.path).expect("WordlistFile should be");
    let mut file_text: String = String::new();
    file.read_to_string(&mut file_text);

    for i in file_text.split('\n').into_iter(){
        let url_code: Result<StatusCode, &str>;
        let url: String;

        if &args.url[args.url.len()-2..args.url.len()-1] == "/"{
            url = format!("{}/{}/", &args.url, i);
            url_code = get_code(&url).await; 
        }else{
            url = format!("{}{}/", &args.url, i);
            url_code = get_code(&url).await;
        }
    
        match url_code{
            Ok(status) => output.push(format!("{} ({})", url, status.to_string())),
            Err(error) => println!("Error: {}", error),
        }
    }
    println!("{:?}", output);
    Ok(())
}
