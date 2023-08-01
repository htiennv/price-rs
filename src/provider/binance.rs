use anyhow::Ok;
use reqwest::blocking::Client;

use crate::config::Config;

use super::{PriceInfo, Provider, Symbol};

pub struct BinanceClient<'a> {
    pub cfg: &'a Config,
    pub client: Client,
}

impl<'a> BinanceClient<'a> {
    pub fn new(cfg: &'a Config, client: Client) -> Self {
        Self { client, cfg }
    }

    pub fn get_prices_helper(&self, symbols: Vec<Symbol>) -> anyhow::Result<Vec<PriceInfo>> {
        Ok(vec![])
    }
}

impl<'a> Provider<'a> for BinanceClient<'a> {
    fn get_prices(&self, symbols: Vec<Symbol>) -> anyhow::Result<Vec<PriceInfo>> {
        Ok(vec![])
    }

    fn get_name(&self) -> &'static str {
        "binance"
    }
}

#[cfg(test)]
mod tests {
    use anyhow::Ok;
    use serde::Deserialize;

    use crate::config::Config;

    use super::BinanceClient;

    #[test]
    fn test_call() {
        let symbols = vec!["BTCUSDT", "BNBUSDT"];

        let cfg = Config::default();
        let client = reqwest::blocking::Client::new();

        let binance = BinanceClient {
            cfg: &cfg,
            client: client,
        };

        let url = format!(
            "{}/ticker/price?symbols={}",
            binance.cfg.binance_url,
            serde_json::to_string(&symbols).unwrap()
        );
        println!("url={url}");

        let res = binance.client.get(url).send();
        assert!(res.is_ok());
        println!("{:?}", res);
    }

    #[derive(Deserialize, Debug, Clone)]
    struct BinancePriceResp {
        symbol: String,
        price: String,
    }

    fn call_basic() -> anyhow::Result<Vec<BinancePriceResp>> {
        let url = r#"https://api.binance.com/api/v3/ticker/price?symbols=["BTCUSDT","BNBUSDT"]"#;

        let response: Vec<BinancePriceResp> = reqwest::blocking::get(url)?.json()?;
        Ok(response)
    }

    #[test]
    fn test_call_basic() {
        let res = call_basic();

        println!("Response: {:?}", res.unwrap());
    }
}
