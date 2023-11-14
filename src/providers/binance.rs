use anyhow::Ok;
use async_trait::async_trait;
use serde::Deserialize;

use crate::config::Config;

use super::{PriceInfo, PriceInfoMap, Provider};

pub struct BinanceClient {
    inner: reqwest::Client,
}

impl BinanceClient {
    pub fn new() -> Self {
        Self {
            inner: reqwest::Client::builder()
                .connection_verbose(true)
                .build()
                .expect("reqwest::Client should have built"),
        }
    }
}

#[derive(Deserialize, Debug, Clone)]
struct BinancePriceResponse {
    symbol: String,
    price: String,
}

fn remove_usdt(s: &str) -> String {
    s.strip_suffix("USDT").unwrap().to_lowercase()
}

#[async_trait]
impl Provider for BinanceClient {
    
    async fn get_prices(&self, config: &Config) -> anyhow::Result<PriceInfoMap> {
        let mut has_usdt = false;

        let mut binance_symbols: Vec<String> = Vec::new();
        for s in &config.symbols {
            if s.symbol == "usdt" {
                has_usdt = true;
                break;
            }
            binance_symbols.push(format!("{}USDT", s.symbol.to_uppercase()));
        }

        let url = format!(
            "{}/ticker/price?symbols={}",
            config.providers.binance.url,
            serde_json::to_string(&binance_symbols).unwrap()
        );
        tracing::info!("calling url: {}", url);
        let res: Vec<BinancePriceResponse> = self.inner.get(url).send().await?.json().await?;

        let mut map = PriceInfoMap::new();
        for e in res {
            let symbol = remove_usdt(&e.symbol);
            map.insert(
                symbol.clone(),
                PriceInfo {
                    symbol,
                    price: e.price.parse().unwrap(),
                    change: None,
                },
            );
        }
        if has_usdt {
            map.insert(
                "usdt".to_owned(),
                PriceInfo {
                    symbol: "usdt".to_owned(),
                    price: 1.0,
                    change: None,
                },
            );
        }
        Ok(map)
    }

    fn get_name(&self, _config: &Config) -> &'static str {
        "binance"
    }

    fn get_weight(&self, config: &Config) -> f64 {
        config.providers.binance.weight
    }
}

#[cfg(test)]
mod tests {
    use crate::{config::Config, providers::Provider};

    use super::BinanceClient;

    #[tokio::test]
    async fn test_call() {
        let config = Config::default();
        let client = BinanceClient::new();

        let res = client.get_prices(&config).await.unwrap();
        assert_eq!(res.len(), config.symbols.len());
        println!("{:?}", res);
    }
}
