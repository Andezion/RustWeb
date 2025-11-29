use std::net::SocketAddr;

mod app;
mod auth;
mod forum;

use crate::app::create_app;
use hyper::Server;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
	tracing_subscriber::fmt::init();

	let _app = create_app().await?;

	tracing::info!("backend created (server start omitted). To run the server, edit main.rs to start Server::bind or use a small wrapper.");

	Ok(())
}

