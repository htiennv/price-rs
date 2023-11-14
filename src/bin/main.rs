use price_rs::{config::Config, providers::Service};
use tokio::signal;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    tokio::spawn(async {
        let config = Config::default();
        let service = Service::new(config);
        service.run().await;
    });

    match signal::ctrl_c().await {
        Ok(()) => {
            println!("received ctrl_c signal");
        },
        Err(err) => {
            eprintln!("Unable to listen for shutdown signal: {}", err);
        },
    }
    
    
}
