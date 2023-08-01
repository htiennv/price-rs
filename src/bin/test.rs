use serde::Deserialize;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // // This `derive` requires the `serde` dependency.
    // #[derive(Deserialize, Debug)]
    // struct Ip {
    //     origin: String,
    // }

    // let json: Ip = reqwest::blocking::get("http://httpbin.org/ip")?.json()?;
    // println!("{:?}", json);

    #[derive(Deserialize, Debug, Clone)]
    struct BinancePriceResp {
        symbol: String,
        price: String,
    }

    let url = r#"https://api.binance.com/api/v3/ticker/price?symbols=["BTCUSDT","BNBUSDT"]"#;
    let response: Vec<BinancePriceResp> = reqwest::blocking::get(url)?.json()?;

    println!("{:?}", response);

    Ok(())
}
