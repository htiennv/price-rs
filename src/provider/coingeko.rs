use serde::{Deserialize, Serialize};

use crate::config::{Config, SymbolInfo};

use super::{Provider, PriceInfoMap};

pub struct CoingeckoClient<'a> {
    pub config: &'a Config,
    pub http_client: reqwest::blocking::Client,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PriceResponse {
    pub symbol: String,
    pub price: String,
}

impl<'a> CoingeckoClient<'a> {
    pub fn new(config: &'a Config, http_client: reqwest::blocking::Client) -> Self {
        Self { config, http_client }
    }
}

impl<'a> Provider for CoingeckoClient<'a> {
    fn get_prices(&self, symbols: Vec<SymbolInfo>) -> anyhow::Result<PriceInfoMap> {
        todo!()
    }

    fn get_name(&self) -> String {
        "coingecko".to_owned()
    }

    fn get_weight(&self) -> f64 {
        self.config.providers.coingecko.weight
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_call() {
    }
}
