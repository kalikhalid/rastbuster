use tokio;
use clap::Parser;
use clap;
use std::fs;
use std::io::Read;
use reqwest::{StatusCode, Error};

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

async fn get_code(url: &str) -> Result<StatusCode, Error> {
    Ok(reqwest::get(url).await?.status())
}

#[tokio::main]
async fn main() -> Result<(), bool> {
    // println!("{}", get_code("https://monkeytype.com/").await.unwrap());

    let s = format!(
        "\nKhalid project\n{}\n{}\n{}\n{}\n\n{}\n\n",
        r#".---.  .-. .-. .----..-----..----. .-. .-. .----..-----..----..---.  "#,
        r#"} }}_} | } { |{ {__-``-' '-'| {_} }| } { |{ {__-``-' '-'} |__}} }}_} "#,
        r#"| } \  \ `-' /.-._} }  } {  | {_} }\ `-' /.-._} }  } {  } '__}| } \  "#,
        r#"`- '-'  `---' `----'   `-'  `----'  `---' `----'   `-'  `----'`-'-'  "#,
        r#"The Modern Day Site Directory Buster."#
    );
    println!("{}", s);

    let mut output: Vec<String> = Vec::new();

    let args = Cli::parse();

    let mut file =  fs::File::open(args.path).expect("WordlistFile should be");
    let mut file_text: String = String::new();
    let _ = file.read_to_string(&mut file_text);

    for i in file_text.split('\n').into_iter(){
        let url_code: Result<StatusCode, Error>;
        let url: String;
        if &args.url[args.url.len()-2..args.url.len()-1] == "/"{
            url = format!("{}/{}/", args.url, i);
            url_code = get_code(&url).await; 
        }else{
            url = format!("{}/{}/", &args.url, i);
            url_code = get_code(&url).await;
        }
    
        match url_code {
                        
            Ok(code) => {
                if !code.to_string().contains("404") && i.len() > 1{
                    println!("/{} {}", i, code)
                }
            },
            Err(e) => {println!("Error: {}", e); return Err(false);},

        }

    }
    Ok(())
}
