use anyhow::Result;
use notify_server::get_router;
use tracing::level_filters::LevelFilter;
use tracing_subscriber::{Layer as _, fmt::Layer, layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> Result<()> {
    let layer = Layer::new().with_filter(LevelFilter::INFO);
    tracing_subscriber::registry().with(layer).init();

    let addr = "0.0.0.0:6687";

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    let app = get_router();

    axum::serve(listener, app).await.unwrap();

    Ok(())
}
