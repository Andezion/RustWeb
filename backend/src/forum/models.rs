use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct ForumTopic {
    pub id: Uuid,
    pub title: String,
    pub author_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForumPost {
    pub id: Uuid,
    pub topic_id: Uuid,
    pub author_id: Option<Uuid>,
    pub content: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateTopicRequest {
    pub title: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreatePostRequest {
    pub topic_id: String,
    pub content: String,
}
