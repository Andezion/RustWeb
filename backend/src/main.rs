use std::net::SocketAddr;

mod app;
mod auth;
mod forum;
mod sports;

use crate::app::create_app;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let app = create_app().await?;

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::info!("Server starting on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

