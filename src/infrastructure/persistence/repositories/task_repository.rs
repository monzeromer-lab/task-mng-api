use crate::domain::task::model::{
    ActiveModel, Column as TaskColumn, Entity as TaskEntity, Model as Task
};
use crate::domain::task::repository::{TaskRepository, TasksFilter};
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sea_orm::{ColumnTrait, DatabaseConnection, DbErr, EntityTrait,JoinType, PaginatorTrait, QueryFilter, QuerySelect, RelationTrait, Set
};

pub struct TaskRepo {
    db: DatabaseConnection,
}

impl TaskRepo {
    pub fn _new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}

#[async_trait]
impl TaskRepository for TaskRepo {
    async fn due_date_datetime(&self) -> DateTime<Utc> {
        Utc::now()
    }

    async fn find_task(&self, task_id: i32) -> Result<Task, DbErr> {
        let query = TaskEntity::find_by_id(task_id).join(JoinType::InnerJoin, crate::domain::task::model::Relation::Users.def()).one(&self.db).await?;
        query.ok_or(DbErr::RecordNotFound("Task Not Found".to_string()))
    }

    async fn find(&self, limit: Option<u8>) -> Result<Vec<Task>, DbErr> {
        let query = TaskEntity::find()
            .limit(limit.unwrap_or(10) as u64)
            .all(&self.db)
            .await?;

        Ok(query)
    }

    async fn filter_tasks(
        &self,
        user_id: i32,
        filter: TasksFilter,
        page_number: u8,
    ) -> Result<Vec<Task>, DbErr> {
        let mut query = TaskEntity::find().filter(TaskColumn::UserId.eq(user_id));

        if !filter.title.is_empty() {
            query = query.filter(TaskColumn::Title.contains(&filter.title));
        }

        if let Some(due_date) = filter.due_date {
            query = query.filter(TaskColumn::DueDate.eq(due_date));
        }

        query = query.filter(TaskColumn::Status.eq(filter.status));

        query.paginate(&self.db, page_number as u64).fetch().await
    }

    async fn create(&self, new_task: Task) -> Result<Task, DbErr> {
        let task = ActiveModel {
            title: Set(new_task.title),
            description: Set(new_task.description),
            due_date: Set(new_task.due_date),
            status: Set(new_task.status),
            user_id: Set(new_task.user_id),
            ..Default::default()
        };

        let record_id = TaskEntity::insert(task).exec(&self.db).await?.last_insert_id;
        self.find_task(record_id).await
    }

    async fn update(&self, task_id: i32, task: Task) -> Result<Task, DbErr> {
        let task_result = TaskEntity::find_by_id(task_id).one(&self.db).await?;
        let mut task_to_update: ActiveModel = task_result.ok_or(DbErr::RecordNotFound("".to_string()))?.into();
        
        task_to_update.title = Set(task.title);
        task_to_update.description = Set(task.description);
        task_to_update.due_date = Set(task.due_date);
        task_to_update.status = Set(task.status);

        TaskEntity::update(task_to_update).exec(&self.db).await

    }

    async fn delete(&self, task_id: i32) -> Result<bool, DbErr> {
        let state = TaskEntity::delete_by_id(task_id).exec(&self.db).await?;
        Ok(state.rows_affected > 0)
    }
}
