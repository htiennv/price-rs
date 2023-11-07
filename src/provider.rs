pub mod binance;
pub mod coingeko;

use std::collections::HashMap;

use anyhow;
use serde::{Deserialize, Serialize};

use crate::config::SymbolInfo;

pub type PriceInfoMap = HashMap<&'static str, PriceInfo>;

pub trait Provider: Sync + Send {
    fn get_prices(&self, symbols: Vec<SymbolInfo>) -> anyhow::Result<PriceInfoMap>;
    fn get_name(&self) -> String;
    fn get_weight(&self) -> f64;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriceInfo {
    pub symbol: String,
    pub price: f64,
    pub change: f64,
}

pub trait Service {
    fn avg_prices(&self, symbols: Vec<SymbolInfo>) -> anyhow::Result<PriceInfoMap>;
    fn all_symbols(&self) -> Vec<SymbolInfo>; 
    fn get_current_price_by_symbol(&self, symbol: String) -> anyhow::Result<PriceInfo>;
    fn get_current_prices(&self) -> anyhow::Result<PriceInfoMap>;
    fn update_current_prices(&mut self, prices: &PriceInfoMap) -> anyhow::Result<()>;
    fn is_servable(&self) -> bool;
}
