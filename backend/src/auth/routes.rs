use crate::auth::DbPool;
use axum::routing::post;
use axum::Router;
use axum::extract::Extension;

use crate::auth::handlers::{login, register};

pub fn router(pool: &DbPool) -> Router {
    Router::new()
        .route("/api/auth/register", post(register))
        .route("/api/auth/login", post(login))
        .layer(Extension(pool.clone()))
}
