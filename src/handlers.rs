use axum::{routing::get, Router};

use crate::app::AppState;

use self::price::get_prices;

pub mod price;

pub fn build_handlers(state: AppState) -> Router<()> {
    let router = Router::new().route("/v1/prices", get(get_prices));
    router.with_state(state)
}
