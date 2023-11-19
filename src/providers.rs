pub mod binance;
pub mod coingeko;

use std::{
    collections::HashMap,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, RwLock,
    },
    time::Duration,
};

use anyhow::{self};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use tokio::time;

use crate::{
    config::Config,
    providers::{binance::BinanceClient, coingeko::CoingeckoClient},
};

// Key is symbol id
pub type PriceInfoMap = HashMap<String, PriceInfo>;

#[async_trait]
pub trait Provider: Send + Sync + 'static {
    async fn get_prices(&self, config: &Config) -> anyhow::Result<PriceInfoMap>;
    fn get_name(&self, config: &Config) -> &'static str;
    fn get_weight(&self, config: &Config) -> f64;
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PriceInfo {
    pub symbol: String,
    pub price: f64,
    pub change: Option<f64>,
}

pub struct Service {
    config: Arc<Config>,
    providers: Vec<Arc<Box<dyn Provider>>>,
    is_servable: AtomicBool,
    // A mutex map store current average prices
    current_prices: Arc<RwLock<PriceInfoMap>>,
}

impl Service {
    pub fn new(config: Config) -> Self {
        tracing::info!("init binance provider");
        let binance = Box::new(BinanceClient::new());

        tracing::info!("init coingecko provider");
        let cgk = Box::new(CoingeckoClient::new());

        let mut providers: Vec<Arc<Box<dyn Provider>>> = Vec::new();
        providers.push(Arc::new(binance));
        providers.push(Arc::new(cgk));

        Self {
            providers,
            config: Arc::new(config),
            is_servable: AtomicBool::new(false),
            current_prices: Arc::new(RwLock::new(PriceInfoMap::new())),
        }
    }

    pub fn is_servable(&self) -> bool {
        self.is_servable.load(Ordering::SeqCst)
    }

    pub fn get_prices(&self) -> anyhow::Result<PriceInfoMap> {
        let current = self.current_prices.read().unwrap();
        Ok(current.clone())
    }

    pub async fn run(&self) {
        tracing::info!("starting run loop service...");
        loop {
            let prices = self.avg_prices().await;

            match prices {
                Ok(prices) => {
                    self.is_servable.store(true, Ordering::SeqCst);
                    let mut guard = self.current_prices.write().unwrap();
                    *guard = prices;
                }
                Err(err) => {
                    tracing::info!("err: {}", err);
                    self.is_servable.store(false, Ordering::SeqCst);
                }
            }
            tracing::info!("sleeping in {}", self.config.worker_interval_secs);
            time::sleep(Duration::from_secs(self.config.worker_interval_secs)).await;
        }
    }

    async fn avg_prices(&self) -> anyhow::Result<PriceInfoMap> {
        let mut handles = vec![];
        for p in &self.providers {
            let provider = Arc::clone(&p);
            let config = Arc::clone(&self.config);
            let handle = tokio::spawn(async move {
                let price = provider.get_prices(&config).await;
                let weight = provider.get_weight(&config);
                (price, weight)
            });
            handles.push(handle);
        }

        let mut ok_prices = vec![];
        for handle in handles {
            let (price_result, weight) = handle.await.unwrap();
            match price_result {
                Ok(price) => ok_prices.push((price, weight)),
                Err(err) => tracing::error!("err: {}", err),
            }
        }
        if ok_prices.len() == 0 {
            return Err(anyhow::anyhow!("no price found"));
        }

        let mut result = PriceInfoMap::new();
        let mut total_weight = 0.0;

        for (_, w) in &ok_prices {
            total_weight += w;
        }
        if total_weight <= 0.0 {
            return Err(anyhow::anyhow!("no price found"));
        }

        for info in self.config.symbols.clone() {
            result.insert(
                info.clone().symbol,
                PriceInfo {
                    symbol: info.clone().symbol,
                    price: 0.0,
                    change: None,
                },
            );
            for (ok_price, w) in &ok_prices {
                if result.contains_key(&info.symbol) {
                    if let Some(val) = result.get_mut(&info.symbol) {
                        val.price += ok_price[&info.symbol].price * w;
                    }
                }
            }
            if let Some(val) = result.get_mut(&info.symbol) {
                val.price /= total_weight;
            }
        }
        tracing::info!("result: {:?}", result);
        Ok(result)
    }
}
