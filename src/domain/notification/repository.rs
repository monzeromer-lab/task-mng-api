use async_trait::async_trait;
use sea_orm::DbErr;
use super::model::{ Model as Notification };

#[async_trait]
pub trait NotificationRepository {
    async fn find_notifications(&self, user_id: i32, page_number: u8) -> Result<Vec<Notification>, DbErr>;
    async fn create_notification(&self, notification: Notification) -> Result<Notification, DbErr>;
    async fn mark_notification_as_read(&self, notification_id: i32) -> Result<Notification, DbErr>;
}
