use axum::routing::{get, post};
use axum::Router;
use sqlx::SqlitePool;

use crate::forum::handlers::{create_post, create_topic, list_posts, list_topics};

pub fn router(pool: &SqlitePool) -> Router {
    Router::new()
        .route("/api/forum/topics", get(list_topics).post(create_topic))
        .route("/api/forum/topics/:id/posts", get(list_posts))
        .route("/api/forum/posts", post(create_post))
        .layer(axum::extract::Extension(pool.clone()))
}
