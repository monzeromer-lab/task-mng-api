use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct UserDto {
    pub username: String,
    pub email: String,
    pub active: bool,
    pub password: String,
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserLoginDto {
    pub email: String,
    pub password: String
}
