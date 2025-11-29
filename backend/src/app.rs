use std::env;

use axum::routing::get;
use axum::Router;
use sqlx::sqlite::SqlitePoolOptions;
use sqlx::SqlitePool;

pub async fn create_app() -> anyhow::Result<Router> {
	let database_url = env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite://./data.db".to_string());

	let pool: SqlitePool = SqlitePoolOptions::new()
		.max_connections(5)
		.connect(&database_url)
		.await?;

	sqlx::query(
		r#"CREATE TABLE IF NOT EXISTS users (
			id TEXT PRIMARY KEY,
			username TEXT NOT NULL UNIQUE,
			email TEXT NOT NULL UNIQUE,
			password_hash TEXT NOT NULL,
			created_at TEXT NOT NULL
		);"#,
	)
	.execute(&pool)
	.await?;

	let auth_routes = crate::auth::routes::router(&pool);

	let app = Router::new()
		.route("/health", get(|| async { "ok" }))
		.merge(auth_routes);

	Ok(app)
}

