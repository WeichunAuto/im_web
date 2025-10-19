use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

pub(crate) mod user;
pub(crate) mod workspace;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub(crate) struct User {
    pub id: i64,
    pub fullname: String,
    pub email: String,

    #[sqlx[default]]
    #[serde(skip)] // 序列化和反序列化时 忽略
    pub password_hash: Option<String>,
    pub ws_id: i64,

    pub create_at: DateTime<Utc>,
}
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub(crate) struct Workspace {
    pub id: i64,
    pub name: String,
    pub owner_id: i64,
    pub create_at: DateTime<Utc>,
}

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub(crate) struct ChatUser {
    pub id: i64,
    pub fullname: String,
    pub email: String,
}

#[derive(Serialize)]
pub(crate) struct UserResponse {
    pub id: u32,
    pub name: String,
    pub active: bool,
}
