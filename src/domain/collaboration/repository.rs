use async_trait::async_trait;
use sea_orm::DbErr;

pub type Collaboration = crate::domain::collaboration::model::Model;

pub struct NewCollaboration {
    pub task_id: i32,
    pub user_id: i32
}

#[async_trait]
pub trait CollaborationRepository {
    fn find_task_collaborators(&self, task_id: i32) -> Result<Vec<Collaboration>, DbErr>;
    fn find_user_collaborations(&self, user_id: i32) -> Result<Vec<Collaboration>, DbErr>;
    fn add(&self, collaboration: NewCollaboration) -> Result<Collaboration, DbErr>; 
    fn remove(&self, collaboration_id: i32) -> Result<bool, DbErr>; 
}