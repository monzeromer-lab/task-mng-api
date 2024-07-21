use chrono::{DateTime, NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct UserDto {
    pub username: String,
    pub email: String,
    pub active: Option<bool>,
    pub password: String,
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserResponseDto {
    pub username: String,
    pub email: String,
    pub active: bool,
    pub created_at: Option<NaiveDateTime>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserLoginDto {
    pub email: String,
    pub password: String
}
