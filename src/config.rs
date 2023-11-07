use std::path::PathBuf;

pub struct Config {
    pub worker_interval_secs: i64,
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
                    binance_id: "eth".to_owned(),
                    coingecko_id: "ethereum".to_owned(),
                },
                SymbolInfo {
                    binance_id: "bnb".to_owned(),
                    coingecko_id: "binancecoin".to_owned(),
                },
                SymbolInfo {
                    binance_id: "btc".to_owned(),
                    coingecko_id: "bitcoin".to_owned(),
                },
                SymbolInfo {
                    binance_id: "doge".to_owned(),
                    coingecko_id: "dogecoin".to_owned(),
                },
                SymbolInfo {
                    binance_id: "shib".to_owned(),
                    coingecko_id: "shiba-inu".to_owned(),
                },
                SymbolInfo {
                    binance_id: "lunc".to_owned(),
                    coingecko_id: "terra-luna".to_owned(),
                },
                SymbolInfo {
                    binance_id: "usdt".to_owned(),
                    coingecko_id: "tether".to_owned(),
                },
            ],
        }
    }
}

pub struct Providers {
    pub threshold_change: f64,
    pub binance: ProviderInfo,
    pub coingecko: ProviderInfo,
    pub okx: ProviderInfo,
}

pub struct ProviderInfo {
    pub url: String,
    pub weight: f64,
}

pub struct SymbolInfo {
    pub binance_id: String,
    pub coingecko_id: String,
}


pub fn parse_config(path: PathBuf) -> anyhow::Result<Config> {
    todo!()
}