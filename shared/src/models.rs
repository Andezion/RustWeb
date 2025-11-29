use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserPublic {
	pub id: Uuid,
	pub username: String,
	pub email: String,
	pub created_at: DateTime<Utc>,
}
