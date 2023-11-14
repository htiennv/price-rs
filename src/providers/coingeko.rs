use std::collections::HashMap;

use anyhow::Ok;
use async_trait::async_trait;
use serde_json::Value;

use crate::config::Config;

use super::{PriceInfo, PriceInfoMap, Provider};

pub struct CoingeckoClient {
    inner: reqwest::Client,
}

impl CoingeckoClient {
    pub fn new() -> Self {
        Self {
            inner: reqwest::Client::builder()
                .connection_verbose(true)
                .build()
                .expect("reqwest::Client should have built"),
        }
    }
}

fn symbol_from_cgk_id(config: &Config, id: &str) -> Option<String> {
    for info in &config.symbols {
        if info.ckg_id == id {
            return Some(info.symbol.clone());
        }
    }
    None
}

#[async_trait]
impl Provider for CoingeckoClient {
    async fn get_prices(&self, config: &Config) -> anyhow::Result<PriceInfoMap> {
        let mut cgk_ids: Vec<String> = Vec::new();
        for s in &config.symbols {
            cgk_ids.push(s.clone().ckg_id);
        }

        let url = format!("{}/simple/price?ids={}&vs_currencies=usd&include_market_cap=false&include_24hr_vol=false&include_24hr_change=false&include_last_updated_at=false", config.providers.coingecko.url, cgk_ids.join(","));
        println!("calling url: {}", url);
        let value: Value = self.inner.get(url).send().await?.json().await?;
        let mut result = PriceInfoMap::new();
        if let Value::Object(map) = value {
            let coin_prices: HashMap<String, f64> = map
                .into_iter()
                .filter_map(|(key, value)| {
                    if let Value::Object(inner_map) = value {
                        if let Some(inner_value) = inner_map.get("usd") {
                            if let Value::Number(price) = inner_value {
                                return Some((key, price.as_f64().unwrap()));
                            }
                        }
                    }
                    None
                })
                .collect();

            // Access the data
            for (coin, price) in coin_prices {
                if let Some(symbol) = symbol_from_cgk_id(&config, &coin) {
                    result.insert(
                        symbol.clone(),
                        PriceInfo {
                            symbol,
                            price,
                            change: None,
                        },
                    );
                }
            }
        }
        Ok(result)
    }

    fn get_name(&self, _config: &Config) -> &'static str {
        "coingecko"
    }

    fn get_weight(&self, config: &Config) -> f64 {
        config.providers.coingecko.weight
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        config::Config,
        providers::{coingeko::CoingeckoClient, Provider},
    };

    #[tokio::test]
    async fn test_call() {
        let config = Config::default();
        let client = CoingeckoClient::new();

        let res = client.get_prices(&config).await.unwrap();
        assert_eq!(res.len(), config.symbols.len());
        println!("{:?}", res);
    }
}
