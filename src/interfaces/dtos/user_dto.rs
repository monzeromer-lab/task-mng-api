use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct UserDto {
    pub username: String,
    pub email: String,
    pub active: bool,
    pub password_hash: String,
    pub created_at: Option<DateTime<Utc>>,
}
