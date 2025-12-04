use crate::sports::models::{CreateRecordRequest, CreateSportRequest, Record, Sport};
use axum::extract::{Extension, Path};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use chrono::Utc;
use sqlx::{Row, SqlitePool};
use uuid::Uuid;

pub async fn list_sports(Extension(pool): Extension<SqlitePool>) -> impl IntoResponse {
    let rows = sqlx::query("SELECT id, name, description FROM sports ORDER BY name")
        .fetch_all(&pool)
        .await;

    match rows {
        Ok(rs) => {
            let sports: Vec<Sport> = rs
                .into_iter()
                .map(|r| {
                    let id: String = r.get("id");
                    let name: String = r.get("name");
                    let description: Option<String> = r.get("description");
                    Sport {
                        id: Uuid::parse_str(&id).unwrap_or_else(|_| Uuid::nil()),
                        name,
                        description,
                    }
                })
                .collect();
            (StatusCode::OK, Json(sports)).into_response()
        }
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to fetch sports: {}", e),
        )
            .into_response(),
    }
}

pub async fn get_sport(
    Extension(pool): Extension<SqlitePool>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let row = sqlx::query("SELECT id, name, description FROM sports WHERE id = ?1")
        .bind(&id)
        .fetch_optional(&pool)
        .await;

    match row {
        Ok(Some(r)) => {
            let id: String = r.get("id");
            let name: String = r.get("name");
            let description: Option<String> = r.get("description");
            let sport = Sport {
                id: Uuid::parse_str(&id).unwrap_or_else(|_| Uuid::nil()),
                name,
                description,
            };
            (StatusCode::OK, Json(sport)).into_response()
        }
        Ok(None) => (StatusCode::NOT_FOUND, "Sport not found").into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Database error: {}", e),
        )
            .into_response(),
    }
}

pub async fn create_sport(
    Extension(pool): Extension<SqlitePool>,
    Json(payload): Json<CreateSportRequest>,
) -> impl IntoResponse {
    let id = Uuid::new_v4();
    let res = sqlx::query("INSERT INTO sports (id, name, description) VALUES (?1, ?2, ?3)")
        .bind(id.to_string())
        .bind(&payload.name)
        .bind(&payload.description)
        .execute(&pool)
        .await;

    match res {
        Ok(_) => (
            StatusCode::CREATED,
            Json(Sport {
                id,
                name: payload.name,
                description: payload.description,
            }),
        )
            .into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to create sport: {}", e),
        )
            .into_response(),
    }
}

pub async fn list_records(
    Extension(pool): Extension<SqlitePool>,
    Path(sport_id): Path<String>,
) -> impl IntoResponse {
    let rows = sqlx::query(
        "SELECT id, sport_id, user_id, value, category, created_at FROM records WHERE sport_id = ?1 ORDER BY created_at DESC"
    )
    .bind(&sport_id)
    .fetch_all(&pool)
    .await;

    match rows {
        Ok(rs) => {
            let records: Vec<Record> = rs
                .into_iter()
                .map(|r| {
                    let id: String = r.get("id");
                    let sport_id: String = r.get("sport_id");
                    let user_id: Option<String> = r.get("user_id");
                    let value: String = r.get("value");
                    let category: Option<String> = r.get("category");
                    Record {
                        id: Uuid::parse_str(&id).unwrap_or_else(|_| Uuid::nil()),
                        sport_id: Uuid::parse_str(&sport_id).unwrap_or_else(|_| Uuid::nil()),
                        user_id: user_id.and_then(|s| Uuid::parse_str(&s).ok()),
                        value,
                        category,
                        created_at: Utc::now(),
                    }
                })
                .collect();
            (StatusCode::OK, Json(records)).into_response()
        }
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to fetch records: {}", e),
        )
            .into_response(),
    }
}

pub async fn create_record(
    Extension(pool): Extension<SqlitePool>,
    Json(payload): Json<CreateRecordRequest>,
) -> impl IntoResponse {
    let id = Uuid::new_v4();
    let created_at = Utc::now().to_rfc3339();
    let res = sqlx::query(
        "INSERT INTO records (id, sport_id, value, category, created_at) VALUES (?1, ?2, ?3, ?4, ?5)",
    )
    .bind(id.to_string())
    .bind(&payload.sport_id)
    .bind(&payload.value)
    .bind(&payload.category)
    .bind(created_at)
    .execute(&pool)
    .await;

    match res {
        Ok(_) => (
            StatusCode::CREATED,
            Json(Record {
                id,
                sport_id: Uuid::parse_str(&payload.sport_id).unwrap_or_else(|_| Uuid::nil()),
                user_id: None,
                value: payload.value,
                category: payload.category,
                created_at: Utc::now(),
            }),
        )
            .into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to create record: {}", e),
        )
            .into_response(),
    }
}
