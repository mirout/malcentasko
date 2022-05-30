use r2d2::State;

use crate::config::Connection;
use crate::errors::ServiceError;
use crate::models::task::{JoinedTask, Task, TaskDescription, TaskInfo};
use crate::models::types::{StatusTable, TypeTable};
use crate::models::user::UserInfo;

pub fn create_task(
    user: UserInfo,
    task: TaskDescription,
    conn: &Connection,
) -> Result<uuid::Uuid, ServiceError> {
    let status = StatusTable::get_status_by_name(&task.status_type, conn).or(
        StatusTable::create_status_by_name(&task.status_type, user.id, conn),
    )?;
    let types = TypeTable::get_type_by_name(&task.task_type, conn).or(
        TypeTable::create_type_by_name(&task.task_type, user.id, conn),
    )?;
    Task::create_new_task(&TaskInfo::with_owner_id(task, status, types, user.id), conn)
}

pub(crate) fn get_users_tasks(
    owner: UserInfo,
    conn: &Connection,
) -> Result<Vec<JoinedTask>, ServiceError> {
    Task::get_users_tasks_with_join(owner.id, conn)
}

pub(crate) fn get_children_tasks(
    owner: UserInfo,
    id: uuid::Uuid,
    conn: &Connection,
) -> Result<Vec<Task>, ServiceError> {
    Task::get_children_tasks(owner.id, id, conn)
}

pub(crate) fn update_task_status(
    owner: UserInfo,
    task_id: uuid::Uuid,
    new_value: String,
    conn: &Connection,
) -> Result<(), ServiceError> {
    let status = StatusTable::get_status_by_name(&new_value, conn).or(
        StatusTable::create_status_by_name(&new_value, owner.id, conn),
    )?;
    Task::set_new_status(task_id, status, conn)
}
