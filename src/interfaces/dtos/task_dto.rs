use chrono::{DateTime, Utc};

use crate::domain::task::model::TaskState;

pub struct TaskDto {
    pub title: String,
    pub description: Option<String>,
    pub due_date: Option<DateTime<Utc>>,
    pub status: TaskState,
    pub user_id: Option<i32>,
}
