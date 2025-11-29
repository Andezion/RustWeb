use crate::auth::models::{AuthResponse, LoginRequest, RegisterRequest, User};
use crate::auth::DbPool;
use axum::extract::Extension;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use chrono::Utc;
use jsonwebtoken::{EncodingKey, Header};
use sqlx::Row;
use uuid::Uuid;
use argon2::Argon2;
use argon2::password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString};

#[derive(serde::Serialize)]
struct Claims {
    sub: String,
    exp: usize,
}

async fn hash_password(password: &str) -> anyhow::Result<String> {
    let salt = SaltString::generate(&mut rand::thread_rng());
    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|e| anyhow::anyhow!(e.to_string()))?
        .to_string();
    Ok(password_hash)
}

async fn verify_password(hash: &str, password: &str) -> anyhow::Result<bool> {
    let parsed = PasswordHash::new(hash).map_err(|e| anyhow::anyhow!(e.to_string()))?;
    Ok(Argon2::default().verify_password(password.as_bytes(), &parsed).is_ok())
}

pub async fn register(
    Extension(pool): Extension<DbPool>,
    Json(payload): Json<RegisterRequest>,
) -> impl IntoResponse {
    if payload.password.len() < 6 {
        return (StatusCode::BAD_REQUEST, "password too short").into_response();
    }

    let id = Uuid::new_v4();
    let created_at = Utc::now();
    let password_hash = match hash_password(&payload.password).await {
        Ok(h) => h,
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, format!("hash err: {}", e)).into_response(),
    };

    let res = sqlx::query(
        r#"INSERT INTO users (id, username, email, password_hash, created_at) VALUES (?1, ?2, ?3, ?4, ?5)"#,
    )
    .bind(id.to_string())
    .bind(&payload.username)
    .bind(&payload.email)
    .bind(&password_hash)
    .bind(created_at.to_rfc3339())
    .execute(&pool)
    .await;

    match res {
        Ok(_) => {
            let user = User { id, username: payload.username, email: payload.email, created_at };
            (StatusCode::CREATED, Json(user)).into_response()
        }
        Err(e) => {
            tracing::error!("insert user error: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, format!("db err: {}", e)).into_response()
        }
    }
}

pub async fn login(
    Extension(pool): Extension<DbPool>,
    Json(payload): Json<LoginRequest>,
) -> impl IntoResponse {
    let row = sqlx::query("SELECT id, password_hash FROM users WHERE email = ?1")
        .bind(&payload.email)
        .fetch_optional(&pool)
        .await;

    let row = match row {
        Ok(Some(r)) => r,
        Ok(None) => return (StatusCode::UNAUTHORIZED, "invalid credentials").into_response(),
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, format!("db err: {}", e)).into_response(),
    };

    let id: String = row.try_get("id").unwrap_or_default();
    let pw_hash: String = row.try_get("password_hash").unwrap_or_default();

    match verify_password(&pw_hash, &payload.password).await {
        Ok(true) => {
            let secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| "dev-secret-change-me".to_string());
            let exp = (chrono::Utc::now() + chrono::Duration::hours(24)).timestamp() as usize;
            let claims = Claims { sub: id.clone(), exp };
            let token = jsonwebtoken::encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_bytes()));
            match token {
                Ok(t) => (StatusCode::OK, Json(AuthResponse { token: t })).into_response(),
                Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, format!("jwt err: {}", e)).into_response(),
            }
        }
        Ok(false) => (StatusCode::UNAUTHORIZED, "invalid credentials").into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, format!("verify err: {}", e)).into_response(),
    }
}
