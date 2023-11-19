use axum::{extract::State, Json};
use reqwest::StatusCode;
use serde_json::{Value, json};

use crate::{providers::PriceInfo, app::AppState};

pub async fn get_prices(State(state): State<AppState>) -> Result<Json<Value>, StatusCode> {
    let service = &state;
    if !service.is_servable() {
        return Err(StatusCode::NOT_FOUND);
    }
    let prices = service.get_prices().unwrap();
    let mut results = Vec::new();
    for (symbol, info) in prices {
        results.push(PriceInfo{ symbol: symbol, price: info.price, change: info.change });
    }
    results.sort_by(|r1, r2| r1.symbol.cmp(&r2.symbol));
    Ok(Json(json!({
        "prices": results,
    })))
}
