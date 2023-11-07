use crate::config::{Config, SymbolInfo};

use super::{Provider, PriceInfoMap};

pub struct Client<'a> {
    pub config: &'a Config,
    pub http_client: reqwest::blocking::Client,
}

impl<'a> Client<'a> {
    pub fn new(config: &'a Config, http_client: reqwest::blocking::Client) -> Self {
        Self { http_client, config }
    }
}

impl<'a> Provider for Client<'a> {
    fn get_prices(&self, symbols: Vec<SymbolInfo>) -> anyhow::Result<PriceInfoMap> {
        todo!()
    }

    fn get_name(&self) -> String{
        "binance".to_owned()
    }

    fn get_weight(&self) -> f64 {
        self.config.providers.binance.weight
    }
}

#[cfg(test)]
mod tests {
    use serde::Deserialize;

    use crate::config::Config;

    use super::Client;

    #[test]
    fn test_call() {
        let symbols = vec!["BTCUSDT", "BNBUSDT"];

        let config = Config::default();
        let http_client = reqwest::blocking::Client::new();

        let binance = Client {
            config: &config,
            http_client,
        };

        let url = format!(
            "{}/ticker/price?symbols={}",
            binance.config.providers.binance.url,
            serde_json::to_string(&symbols).unwrap()
        );
        println!("url={url}");

        let res: Vec<BinancePriceResponse> = binance.http_client.get(url).send().unwrap().json().unwrap();
        assert_eq!(res.len(), 2);
        println!("{:?}", res);
    }

    #[derive(Deserialize, Debug, Clone)]
    struct BinancePriceResponse {
        symbol: String,
        price: String,
    }
}
