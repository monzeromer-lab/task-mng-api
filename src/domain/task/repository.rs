// repository trait

use super::model::TaskState;


pub type Task = crate::domain::task::model::Model;

pub struct TasksFilter {
    pub title: String,
    pub due_date: Option<String>,
    pub status: TaskState,
}

pub trait TaskRepository {
    fn find_task(task_id: i32) -> Task;
    fn find(limit: Option<u8>) -> Vec<Task>;
    fn filter_tasks(user_id: i32, filter: TasksFilter, page_number: u8) -> Vec<Task>;
    fn create(task: Task) -> Task;
    fn update(task: Task, task_id: i32) -> Task;
    fn delete(task_id: i32) -> bool;
}
