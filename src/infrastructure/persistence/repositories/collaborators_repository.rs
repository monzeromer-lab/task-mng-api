use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
use sea_orm::DbErr;
use crate::domain::collaboration::model::{ Entity as CollaborationEntity, Model as Collaboration, Column, ActiveModel};
use crate::domain::collaboration::repository::CollaborationRepository;

pub struct CollaborationRepo {
    db: DatabaseConnection
}

impl CollaborationRepo {
    fn _new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}

impl CollaborationRepository for CollaborationRepo {
    async fn find_task_collaborators(&self, task_id: i32) -> Result<Vec<Collaboration>, DbErr> {
        CollaborationEntity::find()
            .filter(Column::TaskId.eq(task_id))
            .all(&self.db)
            .await
    }

    async fn find_user_collaborations(&self, user_id: i32) -> Result<Vec<Collaboration>, DbErr> {
        CollaborationEntity::find()
            .filter(Column::UserId.eq(user_id))
            .all(&self.db)
            .await
    }

    async fn add(&self, collaboration: Collaboration) -> Result<Collaboration, DbErr> {
        let collaborator = ActiveModel {
            user_id: Set(collaboration.user_id),
            task_id: Set(collaboration.task_id),
            ..Default::default()
        };
        // Lol
        let collabotator_record = CollaborationEntity::insert(collaborator).exec(&self.db).await?.last_insert_id;
        CollaborationEntity::find_by_id(collabotator_record).one(&self.db).await?.ok_or(DbErr::RecordNotFound("Couldn't find the collaborator".to_string()))
    }

    async fn remove(&self, collaboration_id: i32) -> Result<bool, DbErr> {
        let record = ActiveModel {
            id: Set(collaboration_id),
            ..Default::default()
        };
        let deleted = CollaborationEntity::delete(record)
            .exec(&self.db)
            .await?;
        
        Ok(deleted.rows_affected > 0)
    }
}
