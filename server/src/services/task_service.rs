use crate::config::Connection;
use crate::errors::ServiceError;
use crate::models::task::{Task, TaskDescription, TaskInfo};
use crate::models::user::UserInfo;

pub fn create_task(
    user: UserInfo,
    task: TaskDescription,
    conn: &Connection,
) -> Result<uuid::Uuid, ServiceError> {
    Task::create_new_task(&TaskInfo::with_owner_id(task, user.id), conn)
}

pub(crate) fn get_users_tasks(owner: UserInfo, conn: &Connection) -> Result<Vec<Task>, ServiceError> {
    Task::get_users_tasks(owner.id, conn)
}

pub(crate) fn get_children_tasks(owner: UserInfo, id: uuid::Uuid, conn: &Connection) -> Result<Vec<Task>, ServiceError> {
    Task::get_children_tasks(owner.id, id, conn)
}

pub(crate) fn update_task_status(task_id: uuid::Uuid, new_value: bool, conn: &Connection) -> Result<(), ServiceError> {
    Task::set_new_is_done_value(task_id, new_value, conn)
}
