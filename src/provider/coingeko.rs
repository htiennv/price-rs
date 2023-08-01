use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};

use crate::config::Config;

use super::{PriceInfo, Provider, Symbol};

pub struct CoingeckoClient<'a> {
    pub cfg: &'a Config,
    pub client: Client,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PriceResponse {
    pub symbol: String,
    pub price: String,
}

impl<'a> CoingeckoClient<'a> {
    pub fn new(cfg: &'a Config, client: Client) -> Self {
        Self { cfg, client }
    }

    pub fn get_prices_helper(&self, symbols: Vec<String>) -> anyhow::Result<Vec<PriceInfo>> {
        Ok(vec![])
    }
}

impl<'a> Provider<'a> for CoingeckoClient<'a> {
    fn get_prices(&self, symbols: Vec<Symbol>) -> anyhow::Result<Vec<PriceInfo>> {
        todo!()
    }

    fn get_name(&self) -> &'static str {
        "coingecko"
    }
}

#[cfg(test)]
mod tests {
    use crate::config::Config;

    use super::CoingeckoClient;

    #[test]
    fn test_call() {
        let symbols = vec!["BTCUSDT", "BNBUSDT"];

        let cfg = Config::default();
        let client = reqwest::blocking::Client::new();

        let cgk_client = CoingeckoClient {
            cfg: &cfg,
            client: client,
        };
    }
}
