use serde_derive::Deserialize;
use serde_derive::Serialize;
use serde_json::Value as JsonValue;

#[derive(Serialize, Deserialize)]
pub struct Currency {
    pub id: String,
    pub name: String,
    pub min_size: String,
}

#[derive(Serialize, Deserialize)]
pub struct Data {
    data: Vec<Currency>,
}

pub struct Api {}

impl Api {
    pub fn new() -> Api {
        Api {}
    }

    async fn request(&self, url: &str) -> String {
        let res_text = reqwest::get(url)
            .await
            .expect("Failed")
            .text()
            .await
            .unwrap_or_else(|_| {
                println!("Failed to get exchange rates");
                std::process::exit(1);
            });

        return res_text;
    }

    pub async fn get_currencies(&self) -> Vec<Currency> {
        let res_text = self.request("https://api.coinbase.com/v2/currencies").await;
        let parsed_json: Data = serde_json::from_str(&res_text).expect("Failed to parse json");

        return parsed_json.data;
    }

    pub async fn get_exchange_rate(&self, base: &String, to: &String, quantity: i64) -> f64 {
        self.is_valid(base, to).await;

        let res_text = self
            .request(&format!(
                "https://api.coinbase.com/v2/exchange-rates?currency={}",
                base.to_uppercase()
            ))
            .await;

        let parsed_json: JsonValue = res_text.parse().unwrap();
        let price_str = match parsed_json["data"]["rates"][to.to_uppercase()].as_str() {
            Some(price) => price,
            None => {
                println!("'{}' Invaild currency!", to);
                std::process::exit(1);
            }
        };

        let mut price: f64 = price_str.parse().unwrap();
        price = price * quantity as f64;

        return price;
    }

    async fn is_valid(&self, base: &String, to: &String) {
        let currencies = self.get_currencies().await;

        let is_base = currencies.iter().any(|i| i.id == base.to_uppercase());
        let is_to = currencies.iter().any(|i| i.id == to.to_uppercase());

        if !is_base {
            println!(
                "'{}' Is an invalid currency!\nSee --help for more information",
                base
            );
            std::process::exit(1);
        } else if !is_to {
            println!(
                "'{}' Is an invalid currency!\nSee --help for more information",
                to
            );
            std::process::exit(1);
        }
    }
}
