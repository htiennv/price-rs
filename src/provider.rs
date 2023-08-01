use anyhow;
use serde::{Deserialize, Serialize};

pub mod binance;
pub mod coingeko;

pub trait Provider<'a>: Sync + Send + 'a {
    fn get_prices(&self, symbols: Vec<Symbol>) -> anyhow::Result<Vec<PriceInfo>>;
    fn get_name(&self) -> &'static str;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Symbol {
    pub from: String,
    pub to: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriceInfo {
    pub symbol: Symbol,
    pub price: f64,
}
