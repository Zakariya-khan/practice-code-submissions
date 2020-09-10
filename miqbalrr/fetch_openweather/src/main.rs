#[macro_use]
extern crate dotenv_codegen;
use dotenv::dotenv;
use std::io::stdin;
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let _apikey: String = dotenv!("OPENWEATHERMAP_APIKEY").to_string();
    println!("{}", &_apikey);
    println!("Input City :");
    let mut city = String::new();
    stdin().read_line(&mut city).ok().expect("No city");

    let client = reqwest::Client::new();
    let weather = client.get("http://api.openweathermap.org/data/2.5/forecast")
        .query(&[("q",&city),("appid",&_apikey)])
        .send()
        .await?
        .text()
        .await?;

    println!("{:?}", weather);

    Ok(())
}
