extern crate serde;

mod cli;
mod help;
mod is_valid;

use cli::Cli;
use colored::Colorize;
use help::help;
use is_valid::is_valid;
use reqwest;
use serde_json::Value as JsonValue;

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let cli = Cli::new();

    if cli.help {
        help();
        std::process::exit(0);
    } else if cli.version {
        println!("Version: {}", VERSION.bright_yellow());
        std::process::exit(0);
    }

    if !cli.base.is_empty() && cli.to.is_empty() || cli.base.is_empty() && cli.to.is_empty() {
        println!("You need to provide move info\nExample: 'forx-rs usd aud'\nSee --help");
        std::process::exit(1);
    }

    if !is_valid(&cli.base) {
        println!("'{}' is not a valid currency!", cli.base);
        std::process::exit(1);
    }
    if !is_valid(&cli.to) {
        println!("'{}' is not a valid currency!", cli.to);
        std::process::exit(1);
    }

    let res_text = reqwest::get(format!(
        "https://api.coinbase.com/v2/exchange-rates?currency={}",
        cli.base.to_uppercase()
    ))
    .await?
    .text()
    .await
    .unwrap();
    let parsed_json: JsonValue = res_text.parse().unwrap();
    let price_str = match parsed_json["data"]["rates"][cli.to.to_uppercase()].as_str() {
        Some(price) => price,
        None => {
            println!("Invaild currency!");
            std::process::exit(1);
        }
    };
    let mut price: f64 = price_str.parse().unwrap();
    price = price * cli.quantity_value as f64;

    println!("{}", price);

    Ok(())
}
