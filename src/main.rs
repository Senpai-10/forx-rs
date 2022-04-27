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
    let price_str = match parsed_json["data"]["rates"]["SAR"].as_str() {
        Some(price) => price,
        None => {
            println!("Invaild currency!");
            std::process::exit(1);
        }
    };

    let price: f64 = price_str.parse().unwrap();

    println!("Price: {}", price);
    println!("Price: {}", price * 2.0);

    Ok(())
}
