use sea_orm::DatabaseConnection;
use crate::infrastructure::persistence::repositories::task_repository::TaskRepo;

use super::repository::TaskRepository;

#[derive(Debug, Clone)]
pub struct TaskService<'a> {
    repository: TaskRepo<'a>,
    db: &'a DatabaseConnection,
}

impl<'a> TaskService<'a> {
    pub fn _new(&self, db: &'a DatabaseConnection) -> Self {
        let repository = TaskRepo {
            db: db
        };
        Self { 
            repository,
            db 
        }
    }

    pub async fn find_task(&self, task_id: i32) -> Result<super::model::Model, sea_orm::DbErr> {
        self.repository.find_task(task_id).await
    }

    pub async fn find(&self, limit: Option<u8>) -> Result<Vec<super::model::Model>, sea_orm::DbErr> {
        self.repository.find(limit).await
    }

    pub async fn filter_tasks(
        &self,
        user_id: i32,
        filter: super::repository::TasksFilter,
        page_number: u8,
    ) -> Result<Vec<super::model::Model>, sea_orm::DbErr> {
        self.repository.filter_tasks(user_id, filter, page_number).await
    }

    pub async fn create(
        &self,
        task: super::model::Model,
    ) -> Result<super::model::Model, sea_orm::DbErr> {
        self.repository.create(task).await
    }

    pub async fn update(
        &self,
        task_id: i32,
        task: super::model::Model,
    ) -> Result<super::model::Model, sea_orm::DbErr> {
        self.repository.update(task_id, task).await
    }

    pub async fn delete(&self, task_id: i32) -> Result<bool, sea_orm::DbErr> {
        self.repository.delete(task_id).await
    }
}
