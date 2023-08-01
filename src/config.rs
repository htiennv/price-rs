pub struct Config {
    pub binance_url: String,
    pub coingecko_url: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            binance_url: "https://api.binance.com/api/v3".to_owned(),
            coingecko_url: "https://api.coingecko.com/api/v3".to_owned(),
        }
    }
}
