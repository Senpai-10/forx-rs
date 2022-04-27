use crate::verbose_print;
use serde_json::Value as JsonValue;

pub async fn is_valid(currency: &String, verbose: bool) -> Result<bool, reqwest::Error> {
    let url = "https://api.coinbase.com/v2/currencies";

    verbose_print(format!("GET request to {}", url), verbose);

    let res_text = reqwest::get(url).await?.text().await.unwrap();
    let parsed_json: JsonValue = res_text.parse().unwrap();
    let data = &parsed_json["data"].as_array().unwrap();

    Ok(data.iter().any(|i| i["id"] == currency.to_uppercase()))
}
