use sea_orm::DatabaseConnection;
use async_trait::async_trait;
use sea_orm::DbErr;
use crate::domain::collaboration::repository::Collaboration;
use crate::domain::collaboration::repository::CollaborationRepository;

pub struct CollaborationRepo {
    db: DatabaseConnection
}

impl CollaborationRepo {
    fn _new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}

#[async_trait]
impl CollaborationRepository for CollaborationRepo {
    fn find_task_collaborators(&self, task_id: i32) -> Result<Vec<Collaboration>, DbErr> {
        todo!()
    }

    fn find_user_collaborations(&self, user_id: i32) -> Result<Vec<Collaboration>, DbErr> {
        todo!()
    }

    fn add(&self, collaboration: crate::domain::collaboration::repository::NewCollaboration) -> Result<Collaboration, DbErr> {
        todo!()
    }

    fn remove(&self, collaboration_id: i32) -> Result<bool, DbErr> {
        todo!()
    }
}