use sea_orm::DatabaseConnection;

use crate::infrastructure::persistence::repositories::collaborators_repository::CollaborationRepo;

use super::repository::CollaborationRepository;

pub struct CollaborationService<'a> {
    repository: CollaborationRepo<'a>,
    db: &'a DatabaseConnection,
}

impl<'a> CollaborationService<'a> {
    pub async fn new(&self, db: &'a DatabaseConnection) -> Self {
        let repository = CollaborationRepo { db };
        Self { repository, db }
    }

    async fn find_task_collaborators(
        &self,
        task_id: i32,
    ) -> Result<Vec<super::model::Model>, sea_orm::DbErr> {
        self.repository.find_task_collaborators(task_id).await
    }

    async fn find_user_collaborations(
        &self,
        user_id: i32,
    ) -> Result<Vec<super::model::Model>, sea_orm::DbErr> {
        self.repository.find_user_collaborations(user_id).await
    }

    async fn add(
        &self,
        collaboration: super::model::Model,
    ) -> Result<super::model::Model, sea_orm::DbErr> {
        self.repository.add(collaboration).await
    }

    async fn remove(&self, collaboration_id: i32) -> Result<bool, sea_orm::DbErr> {
        self.repository.remove(collaboration_id).await
    }
}
