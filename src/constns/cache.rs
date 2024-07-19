use crate::domain::task::repository::TasksFilter;
// use std::default;
use crate::domain::user::model::{Model as User};
use crate::domain::task::model::{Model as Task};
use crate::domain::collaboration::model::{Model as Collaboration};
use crate::domain::notification::model::{Model as Notification};

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum CACHE_VALUES {
    USER(User),
    USER_BY_EMAIL(String),
    USER_BY_USERNAME(String),
    USER_COLLABORATIONS(Collaboration),
    TASK(Task),
    TASK_COLLABORATORS(i32),
    TASK_BY_ID(i32),
    TASK_FILTER(TasksFilter),
    NOTIFICATION(Notification),
    NOTIFICATION_FOR_USER(i32),
    VERIFICATION_CODE(String, i32),
}
