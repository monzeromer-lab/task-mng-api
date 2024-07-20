use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct NotificationDto {
    pub user_id: Option<i32>,
    pub task_id: Option<i32>,
    pub message: String,
    pub read: bool,
}
