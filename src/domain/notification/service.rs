use sea_orm::DatabaseConnection;
use crate::infrastructure::persistence::repositories::notification_repository::NotificationRepo;
use super::repository::NotificationRepository;

pub struct NotificationService<'a> {
    repository: NotificationRepo<'a>,
    db: &'a DatabaseConnection
}

impl<'a> NotificationService<'a> {
    pub fn new(&self, db: &'a DatabaseConnection) -> Self {
        let repository = NotificationRepo {
            db
        };
        Self {
            repository,
            db
        }
    }

    async fn find_notifications(
        &self,
        user_id: i32,
        page_number: u8,
    ) -> Result<Vec<super::model::Model>, sea_orm::DbErr> {
        self.repository.find_notifications(user_id, page_number).await
    }

    async fn create_notification(&self, notification: super::model::Model) -> Result<super::model::Model, sea_orm::DbErr> {
        self.repository.create_notification(notification).await
    }

    async fn mark_notification_as_read(&self, notification_id: i32) -> Result<super::model::Model, sea_orm::DbErr> {
        self.repository.mark_notification_as_read(notification_id).await
    }
}
