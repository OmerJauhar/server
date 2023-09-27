use std::error::Error;
use reqwest;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let url = "http://www.google.com/";
    let response = reqwest::get(url).await?; // Use await to await the response.

    let content = response.text().await?; // Use await to await the text content.

    println!("{}", content);

    Ok(())
}
