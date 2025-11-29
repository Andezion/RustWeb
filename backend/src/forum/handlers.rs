use crate::forum::models::{CreatePostRequest, CreateTopicRequest, ForumPost, ForumTopic};
use axum::extract::Extension;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use chrono::Utc;
use sqlx::Row;
use sqlx::SqlitePool;
use uuid::Uuid;

pub async fn list_topics(Extension(pool): Extension<SqlitePool>) -> impl IntoResponse {
    let rows = sqlx::query("SELECT id, title, author_id, created_at FROM forum_topics ORDER BY created_at DESC")
        .fetch_all(&pool)
        .await;

    match rows {
        Ok(rs) => {
            let topics: Vec<ForumTopic> = rs
                .into_iter()
                .map(|r| {
                    let id: String = r.get("id");
                    let title: String = r.get("title");
                    let author: Option<String> = r.get("author_id");
                    let created_at: String = r.get("created_at");
                    ForumTopic {
                        id: Uuid::parse_str(&id).unwrap_or_else(|_| Uuid::nil()),
                        title,
                        author_id: author.and_then(|s| Uuid::parse_str(&s).ok()),
                        created_at: Utc::now(), // simple: not parsing stored time for now
                    }
                })
                .collect();
            (StatusCode::OK, Json(topics)).into_response()
        }
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, format!("db err: {}", e)).into_response(),
    }
}

pub async fn create_topic(
    Extension(pool): Extension<SqlitePool>,
    Json(payload): Json<CreateTopicRequest>,
) -> impl IntoResponse {
    let id = Uuid::new_v4();
    let created_at = Utc::now().to_rfc3339();
    let res = sqlx::query("INSERT INTO forum_topics (id, title, created_at) VALUES (?1, ?2, ?3)")
        .bind(id.to_string())
        .bind(&payload.title)
        .bind(created_at)
        .execute(&pool)
        .await;

    match res {
        Ok(_) => (StatusCode::CREATED, Json(ForumTopic {
            id,
            title: payload.title,
            author_id: None,
            created_at: Utc::now(),
        })).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, format!("db err: {}", e)).into_response(),
    }
}

pub async fn list_posts(
    Extension(pool): Extension<SqlitePool>,
    axum::extract::Path(topic_id): axum::extract::Path<String>,
) -> impl IntoResponse {
    let rows = sqlx::query("SELECT id, topic_id, author_id, content, created_at FROM forum_posts WHERE topic_id = ?1 ORDER BY created_at ASC")
        .bind(&topic_id)
        .fetch_all(&pool)
        .await;

    match rows {
        Ok(rs) => {
            let posts: Vec<ForumPost> = rs
                .into_iter()
                .map(|r| {
                    let id: String = r.get("id");
                    let topic: String = r.get("topic_id");
                    let author: Option<String> = r.get("author_id");
                    let content: String = r.get("content");
                    ForumPost {
                        id: Uuid::parse_str(&id).unwrap_or_else(|_| Uuid::nil()),
                        topic_id: Uuid::parse_str(&topic).unwrap_or_else(|_| Uuid::nil()),
                        author_id: author.and_then(|s| Uuid::parse_str(&s).ok()),
                        content,
                        created_at: Utc::now(),
                    }
                })
                .collect();
            (StatusCode::OK, Json(posts)).into_response()
        }
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, format!("db err: {}", e)).into_response(),
    }
}

pub async fn create_post(
    Extension(pool): Extension<SqlitePool>,
    Json(payload): Json<CreatePostRequest>,
) -> impl IntoResponse {
    let id = Uuid::new_v4();
    let created_at = Utc::now().to_rfc3339();
    let res = sqlx::query("INSERT INTO forum_posts (id, topic_id, content, created_at) VALUES (?1, ?2, ?3, ?4)")
        .bind(id.to_string())
        .bind(&payload.topic_id)
        .bind(&payload.content)
        .bind(created_at)
        .execute(&pool)
        .await;

    match res {
        Ok(_) => (StatusCode::CREATED, Json(ForumPost {
            id,
            topic_id: Uuid::parse_str(&payload.topic_id).unwrap_or_else(|_| Uuid::nil()),
            author_id: None,
            content: payload.content,
            created_at: Utc::now(),
        })).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, format!("db err: {}", e)).into_response(),
    }
}
