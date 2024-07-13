pub type Notification = crate::domain::notification::model::Model;

pub struct NewNotification {
    pub user_id: Option<i32>,
    pub task_id: Option<i32>,
    pub message: String,
    pub read: bool,
}

pub trait NotificationRepository {
    fn find_notifications(user_id: i32, page_number: u8) -> Vec<Notification>;
    fn create_notification(notification: NewNotification) -> Notification;
    fn mark_notification_as_read(notification_id: i32) -> bool;
}