use serde::Deserialize;


#[derive(Debug, Deserialize)]
pub struct CollaborationDto {
    pub user_id: Option<i32>,
    pub task_id: Option<i32>,
}