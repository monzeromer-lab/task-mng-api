pub type User = crate::domain::user::model::Model;

pub trait UserRepository {
    fn signup(user: User) -> User;
    fn find_by_email(email: String) -> User;
    fn find_by_username(email: String) -> User;
    fn update_password(new_password: String, verify_code: String) -> bool;
    fn active_user(user_id: i32, verify_code: String) -> bool;
}