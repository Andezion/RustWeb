use axum::routing::{get, post};
use axum::Router;
use sqlx::SqlitePool;

use crate::sports::handlers::{create_record, create_sport, get_sport, list_records, list_sports};

pub fn router(pool: &SqlitePool) -> Router {
    Router::new()
        .route("/api/sports", get(list_sports).post(create_sport))
        .route("/api/sports/:id", get(get_sport))
        .route("/api/sports/:id/records", get(list_records).post(create_record))
        .layer(axum::extract::Extension(pool.clone()))
}
