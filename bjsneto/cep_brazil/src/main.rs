use std::collections::HashMap;
use std::io::stdin;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {    
    println!("Input CEP Brazil:");
    let mut cep = String::new();
    stdin().read_line(&mut cep).ok().expect("No CEP");
    let url = format!("https://viacep.com.br/ws/{}/json/", cep);
    let resp = reqwest::get(&url)
        .await?
        .json::<HashMap<String, String>>()
        .await?;
    println!("{:#?}", resp);
    Ok(())
  

}