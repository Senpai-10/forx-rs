extern crate serde;

mod api;
mod cli;
mod help;

use api::Api;
use cli::Cli;
use colored::Colorize;
use help::help;
use reqwest;

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let api = Api::new();
    let cli = Cli::new();

    if cli.help {
        help();
        std::process::exit(0);
    } else if cli.version {
        println!("Version: {}", VERSION.bright_yellow());
        std::process::exit(0);
    } else if cli.list {
        let currencies = api.get_currencies().await;

        for i in 0..currencies.len() {
            let currency = &currencies[i];
            let id = &currency.id;
            let name = &currency.name;

            if i % 2 == 1 {
                println!("{}  -  {}", id.bright_black(), name.bright_black());
            } else {
                println!("{}  -  {}", id, name);
            }
        }

        std::process::exit(0);
    }

    let price = api
        .get_exchange_rate(&cli.base, &cli.to, cli.quantity_value)
        .await;

    if cli.no_format {
        println!("{}", price);
    } else {
        println!("{:.2}", price);
    }

    Ok(())
}
