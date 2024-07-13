use chrono::{DateTime, Utc};
use sea_orm::DbErr;
use super::model::{TaskState, Model as Task};

pub struct TasksFilter {
    pub title: String,
    pub due_date: Option<DateTime<Utc>>,
    pub status: TaskState,
}

pub trait TaskRepository {
    async fn due_date_datetime(&self) -> DateTime<Utc>;
    async fn find_task(&self, task_id: i32) -> Result<Task, DbErr>;
    async fn find(&self, limit: Option<u8>) -> Result<Vec<Task>, DbErr>;
    async fn filter_tasks(&self, user_id: i32, filter: TasksFilter, page_number: u8) -> Result<Vec<Task>, DbErr>;
    async fn create(&self, task: Task) -> Result<Task, DbErr>;
    async fn update(&self, task_id: i32, task: Task) -> Result<Task, DbErr>;
    async fn delete(&self, task_id: i32) -> Result<bool, DbErr>;
}
