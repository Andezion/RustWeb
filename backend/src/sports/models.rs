use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Sport {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateSportRequest {
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Record {
    pub id: Uuid,
    pub sport_id: Uuid,
    pub user_id: Option<Uuid>,
    pub value: String,
    pub category: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateRecordRequest {
    pub sport_id: String,
    pub value: String,
    pub category: Option<String>,
}
