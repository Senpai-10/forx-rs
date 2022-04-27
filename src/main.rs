extern crate serde;

mod cli;
mod help;
mod is_valid;
mod verbose_print;

use cli::Cli;
use colored::Colorize;
use help::help;
use is_valid::is_valid;
use reqwest;
use serde_json::Value as JsonValue;
use verbose_print::verbose_print;

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
    } else if cli.list {
        let url = "https://api.coinbase.com/v2/currencies";

        verbose_print(format!("GET request to {}", url), cli.verbose);

        let res_text = reqwest::get(url).await?.text().await.unwrap();
        let parsed_json: JsonValue = res_text.parse().unwrap();
        let data = &parsed_json["data"].as_array().unwrap();

        for i in 0..data.len() {
            let currency = &data[i];
            let id = currency["id"].as_str().unwrap();
            let name = currency["name"].as_str().unwrap();
            if i % 2 == 1 {
                println!("{}  -  {}", id.bright_black(), name.bright_black());
            } else {
                println!("{}  -  {}", id, name);
            }
        }

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

    if cli.verbose {
        println!(
            "{} {} to {} = {}",
            cli.quantity_value, cli.base, cli.to, price
        );
    } else {
        println!("{}", price);
    }

    Ok(())
}
