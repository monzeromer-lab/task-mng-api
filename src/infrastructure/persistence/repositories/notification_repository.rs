use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, DbErr, EntityTrait, PaginatorTrait, QueryFilter, Set};
use async_trait::async_trait;
use crate::domain::notification::model::{Entity as NotificationEntity, Model as Notification, Column, ActiveModel};
use crate::domain::notification::repository::NotificationRepository;

pub struct NotificationRepo {
    db: DatabaseConnection,
}

impl NotificationRepo {
    pub fn _new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}

#[async_trait]
impl NotificationRepository for NotificationRepo {
    async fn find_notifications(&self, user_id: i32, page_number: u8) -> Result<Vec<crate::domain::notification::model::Model>, sea_orm::DbErr> {
        NotificationEntity::find()
            .filter(Column::UserId.eq(user_id))
            .paginate(&self.db, page_number as u64)
            .fetch()
            .await
    }

    async fn create_notification(&self, notification: Notification) -> Result<crate::domain::notification::model::Model, sea_orm::DbErr> {
        let new_notification = ActiveModel {
            user_id: Set(notification.user_id),
            task_id: Set(notification.task_id),
            message: Set(notification.message),
            ..Default::default()
        };
        new_notification.insert(&self.db).await
    }

    async fn mark_notification_as_read(&self, notification_id: i32) -> Result<Notification, sea_orm::DbErr> {
        let notification_record = NotificationEntity::find_by_id(notification_id).one(&self.db).await?;
        let mut notification: ActiveModel = notification_record.ok_or(DbErr::RecordNotFound("No Such a Notification".to_string()))?.into();
        
        notification.read = Set(true);
        
        notification.update(&self.db).await
        
    }
}
