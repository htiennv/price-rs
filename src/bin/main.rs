use std::sync::Arc;

use price_rs::{app::AppState, config::Config, handlers::build_handlers, providers::Service};
use tokio::signal;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let config = Config::default();
    let service = Service::new(config);

    let app = AppState::new(service);

    tokio::spawn(worker(Arc::clone(&app)));
    tokio::spawn(server(Arc::clone(&app)));

    match signal::ctrl_c().await {
        Ok(()) => {
            tracing::info!("received ctrl_c signal");
        }
        Err(err) => {
            tracing::error!("unable to listen for shutdown signal: {}", err);
        }
    }
}

async fn worker(app: AppState) {
    app.run().await
}

async fn server(app: AppState) {
    let axum_router = build_handlers(app);
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(axum_router.into_make_service())
        .await
        .unwrap();
}
