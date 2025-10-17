use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

pub(crate) mod user;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub(crate) struct User {
    pub id: i64,
    pub fullname: String,
    pub email: String,

    #[sqlx[default]]
    #[serde(skip)] // 序列化和反序列化时 忽略
    pub password_hash: Option<String>,

    pub create_at: DateTime<Utc>,
}

#[derive(Serialize)]
pub(crate) struct UserResponse {
    pub id: u32,
    pub name: String,
    pub active: bool,
}
