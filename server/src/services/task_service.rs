use crate::config::Connection;
use crate::errors::ServiceError;
use crate::models::task::{TaskDescription, Task, TaskInfo};
use crate::models::user::UserInfo;

pub fn create_task(user: UserInfo, task: TaskDescription, conn: &Connection) -> Result<uuid::Uuid, ServiceError> {
    Task::create_new_task(&TaskInfo::with_owner_id(task, user.id), conn)
}