use anyhow::Result;
use auth_service::{app::create_app, config::config::load_config};

#[tokio::main]
async fn main() -> Result<()> {
    let config = load_config().await?;
    let app = create_app().await;

    let listener =
        tokio::net::TcpListener::bind(format!("{}:{}", config.app.host, config.app.port)).await?;

    axum::serve(listener, app).await?;
    Ok(())
}
