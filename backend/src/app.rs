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

    sqlx::query(r#"CREATE TABLE IF NOT EXISTS forum_topics (
        id TEXT PRIMARY KEY,
        title TEXT NOT NULL,
        author_id TEXT,
        created_at TEXT NOT NULL
    );"#)
    .execute(&pool)
    .await?;

    sqlx::query(r#"CREATE TABLE IF NOT EXISTS forum_posts (
        id TEXT PRIMARY KEY,
        topic_id TEXT NOT NULL,
        author_id TEXT,
        content TEXT NOT NULL,
        created_at TEXT NOT NULL
    );"#)
    .execute(&pool)
    .await?;

    sqlx::query(r#"CREATE TABLE IF NOT EXISTS sports (
        id TEXT PRIMARY KEY,
        name TEXT NOT NULL UNIQUE,
        description TEXT
    );"#)
    .execute(&pool)
    .await?;

    sqlx::query(r#"CREATE TABLE IF NOT EXISTS records (
        id TEXT PRIMARY KEY,
        sport_id TEXT NOT NULL,
        user_id TEXT,
        value TEXT NOT NULL,
        category TEXT,
        created_at TEXT NOT NULL
    );"#)
    .execute(&pool)
    .await?;

    let auth_routes = crate::auth::routes::router(&pool);
    let forum_routes = crate::forum::routes::router(&pool);
    let sports_routes = crate::sports::routes::router(&pool);

    let app = Router::new()
        .route("/health", get(|| async { "ok" }))
        .merge(auth_routes)
        .merge(forum_routes)
        .merge(sports_routes);

    Ok(app)
}

