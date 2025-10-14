use anyhow::Result;
use chat_server::{config::ServerConfig, routes, AppConfig};
use tracing::level_filters::LevelFilter;
use tracing_subscriber::{fmt::Layer, layer::SubscriberExt, util::SubscriberInitExt, Layer as _};

#[tokio::main]
async fn main() -> Result<()> {
    let layer = Layer::new().with_filter(LevelFilter::INFO);
    tracing_subscriber::registry().with(layer).init();

    let app_config = AppConfig::load().ok().unwrap();

    let app = routes::create_routes(app_config.clone());

    let &ServerConfig { ref host, port } = &app_config.server_config;

    let listener = tokio::net::TcpListener::bind(format!("{}:{}", host, port))
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
