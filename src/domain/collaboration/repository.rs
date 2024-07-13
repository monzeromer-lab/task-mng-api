pub type Collaboration = crate::domain::collaboration::model::Model;

pub struct NewCollaboration {
    pub task_id: i32,
    pub user_id: i32
}

pub trait CollaborationRepository {
    fn find_task_collaborators(task_id: i32) -> Vec<Collaboration>;
    fn find_user_collaborations(user_id: i32) -> Vec<Collaboration>;
    fn add(collaboration: NewCollaboration) -> Collaboration; 
    fn remove(collaboration_id: i32) -> bool; 
}