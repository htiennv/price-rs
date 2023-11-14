use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct Config {
    pub worker_interval_secs: u64,
    pub http_port: String,
    pub providers: Providers,
    pub symbols: Vec<SymbolInfo>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            worker_interval_secs: 30,
            http_port: "8087".to_owned(),
            providers: Providers {
                threshold_change: 10.0,
                binance: ProviderInfo {
                    url: "https://api.binance.com/api/v3".to_owned(),
                    weight: 1.0,
                },
                coingecko: ProviderInfo {
                    url: "https://api.coingecko.com/api/v3".to_owned(),
                    weight: 1.0,
                },
                okx: ProviderInfo {
                    url: "https://www.okx.com/priapi/v5".to_owned(),
                    weight: 1.0,
                },
            },
            symbols: vec![
                SymbolInfo {
                    symbol: "eth".to_owned(),
                    ckg_id: "ethereum".to_owned(),
                },
                SymbolInfo {
                    symbol: "bnb".to_owned(),
                    ckg_id: "binancecoin".to_owned(),
                },
                SymbolInfo {
                    symbol: "btc".to_owned(),
                    ckg_id: "bitcoin".to_owned(),
                },
                SymbolInfo {
                    symbol: "doge".to_owned(),
                    ckg_id: "dogecoin".to_owned(),
                },
                SymbolInfo {
                    symbol: "shib".to_owned(),
                    ckg_id: "shiba-inu".to_owned(),
                },
                SymbolInfo {
                    symbol: "lunc".to_owned(),
                    ckg_id: "terra-luna".to_owned(),
                },
                SymbolInfo {
                    symbol: "usdt".to_owned(),
                    ckg_id: "tether".to_owned(),
                },
            ],
        }
    }
}

#[derive(Debug, Clone)]
pub struct Providers {
    pub threshold_change: f64,
    pub binance: ProviderInfo,
    pub coingecko: ProviderInfo,
    pub okx: ProviderInfo,
}

#[derive(Debug, Clone)]
pub struct ProviderInfo {
    pub url: String,
    pub weight: f64,
}

#[derive(Debug, Clone)]
pub struct SymbolInfo {
    // Symbol is id of a coin in lower case
    pub symbol: String,
    pub ckg_id: String,
}

pub fn parse_config(_path: PathBuf) -> anyhow::Result<Config> {
    todo!()
}
