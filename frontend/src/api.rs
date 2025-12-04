use gloo_net::http::Request;
use serde::{Deserialize, Serialize};

const API_BASE: &str = "http://localhost:3000";

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RegisterRequest {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AuthResponse {
    pub token: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub id: String,
    pub username: String,
    pub email: String,
    pub created_at: String,
}

pub async fn register(req: RegisterRequest) -> Result<User, String> {
    let response = Request::post(&format!("{}/api/auth/register", API_BASE))
        .json(&req)
        .map_err(|e| e.to_string())?
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if response.ok() {
        response.json().await.map_err(|e| e.to_string())
    } else {
        Err(format!("Registration failed: {}", response.status()))
    }
}

pub async fn login(req: LoginRequest) -> Result<AuthResponse, String> {
    let response = Request::post(&format!("{}/api/auth/login", API_BASE))
        .json(&req)
        .map_err(|e| e.to_string())?
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if response.ok() {
        response.json().await.map_err(|e| e.to_string())
    } else {
        Err(format!("Login failed: {}", response.status()))
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ForumTopic {
    pub id: String,
    pub title: String,
    pub author_id: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateTopicRequest {
    pub title: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ForumPost {
    pub id: String,
    pub topic_id: String,
    pub author_id: Option<String>,
    pub content: String,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreatePostRequest {
    pub topic_id: String,
    pub content: String,
}

pub async fn get_topics() -> Result<Vec<ForumTopic>, String> {
    let response = Request::get(&format!("{}/api/forum/topics", API_BASE))
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if response.ok() {
        response.json().await.map_err(|e| e.to_string())
    } else {
        Err(format!("Failed to fetch topics: {}", response.status()))
    }
}

pub async fn create_topic(req: CreateTopicRequest) -> Result<ForumTopic, String> {
    let response = Request::post(&format!("{}/api/forum/topics", API_BASE))
        .json(&req)
        .map_err(|e| e.to_string())?
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if response.ok() {
        response.json().await.map_err(|e| e.to_string())
    } else {
        Err(format!("Failed to create topic: {}", response.status()))
    }
}

pub async fn get_posts(topic_id: &str) -> Result<Vec<ForumPost>, String> {
    let response = Request::get(&format!("{}/api/forum/topics/{}/posts", API_BASE, topic_id))
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if response.ok() {
        response.json().await.map_err(|e| e.to_string())
    } else {
        Err(format!("Failed to fetch posts: {}", response.status()))
    }
}

pub async fn create_post(req: CreatePostRequest) -> Result<ForumPost, String> {
    let response = Request::post(&format!("{}/api/forum/posts", API_BASE))
        .json(&req)
        .map_err(|e| e.to_string())?
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if response.ok() {
        response.json().await.map_err(|e| e.to_string())
    } else {
        Err(format!("Failed to create post: {}", response.status()))
    }
}
