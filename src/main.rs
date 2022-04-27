extern crate serde;

use colored::Colorize;
use reqwest;
use serde_json::Value as JsonValue;

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let res_text = reqwest::get("https://api.coinbase.com/v2/exchange-rates?currency=USD")
        .await?
        .text()
        .await
        .unwrap();

    let parsed_json: JsonValue = res_text.parse().unwrap();
    let price = parsed_json["data"]["rates"]["SAR"].as_str().unwrap();

    println!("Price: {}", price);

    Ok(())
}
