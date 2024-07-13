use sea_orm::DbErr;
use crate::domain::collaboration::model::Model as Collaboration;

pub trait CollaborationRepository {
    async fn find_task_collaborators(&self, task_id: i32) -> Result<Vec<Collaboration>, DbErr>;
    async fn find_user_collaborations(&self, user_id: i32) -> Result<Vec<Collaboration>, DbErr>;
    async fn add(&self, collaboration: Collaboration) -> Result<Collaboration, DbErr>; 
    async fn remove(&self, collaboration_id: i32) -> Result<bool, DbErr>; 
}
