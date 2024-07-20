use crate::domain::collaboration::model::{
    ActiveModel, Column, Entity as CollaborationEntity, Model as Collaboration,
};
use crate::domain::collaboration::repository::CollaborationRepository;
use sea_orm::DbErr;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
use tracing::{event, instrument, Level};

#[derive(Debug)]
pub struct CollaborationRepo<'b> {
    pub db: &'b DatabaseConnection,
}

impl<'b> CollaborationRepo<'b> {
    fn _new(db: &'b DatabaseConnection) -> Self {
        Self { db }
    }
}


impl<'b> CollaborationRepository for CollaborationRepo<'b> {
    #[instrument(level= Level::INFO, name="Collaboration Repository")]
    async fn find_task_collaborators(&self, task_id: i32) -> Result<Vec<Collaboration>, DbErr> {
        CollaborationEntity::find()
            .filter(Column::TaskId.eq(task_id))
            .all(self.db)
            .await
    }
    
    #[instrument(level= Level::INFO, name="Collaboration Repository")]
    async fn find_user_collaborations(&self, user_id: i32) -> Result<Vec<Collaboration>, DbErr> {
        CollaborationEntity::find()
            .filter(Column::UserId.eq(user_id))
            .all(self.db)
            .await
    }
    
    #[instrument(level= Level::INFO, name="Collaboration Repository")]
    async fn add(&self, collaboration: Collaboration) -> Result<Collaboration, DbErr> {
        
        let collaborator = ActiveModel {
            user_id: Set(collaboration.user_id),
            task_id: Set(collaboration.task_id),
            ..Default::default()
        };
        let collaborator_info = format!("{:?}", collaboration);
        event!(Level::INFO, "Adding Collaborator: {collaborator_info}");
        
        let collabotator_record = CollaborationEntity::insert(collaborator)
            .exec(self.db)
            .await?
            .last_insert_id;
        event!(Level::INFO, "Added Collaborator with Id: {collabotator_record}");
        
        CollaborationEntity::find_by_id(collabotator_record)
            .one(self.db)
            .await?
            .ok_or(DbErr::RecordNotFound(
                "Couldn't find the collaborator".to_string(),
            ))
    }

    #[instrument(level= Level::INFO, name="Collaboration Repository")]
    async fn remove(&self, collaboration_id: i32) -> Result<bool, DbErr> {
        let record = ActiveModel {
            id: Set(collaboration_id),
            ..Default::default()
        };
        let deleted = CollaborationEntity::delete(record).exec(self.db).await?;

        Ok(deleted.rows_affected > 0)
    }
}
