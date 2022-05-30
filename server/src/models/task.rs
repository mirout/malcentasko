use crate::{
    config::Connection,
    errors::ServiceError,
    models::user::User,
    schema::{
        task_status_for_user, task_types_for_user,
        tasks::{self, dsl::*},
    },
};
use chrono::{DateTime, Utc};
use diesel::prelude::*;

use super::types::StatusTable;
use super::types::TypeTable;

#[derive(Debug, Identifiable, Associations, Queryable, Serialize, Deserialize)]
#[belongs_to(User)]
pub struct Task {
    pub id: uuid::Uuid,
    pub user_id: uuid::Uuid,
    pub parent_id: Option<uuid::Uuid>,
    pub title: String,
    pub task_description: String,
    pub created_at: DateTime<Utc>,
    pub status_name: Option<uuid::Uuid>,
    pub type_name: Option<uuid::Uuid>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JoinedTask {
    pub id: uuid::Uuid,
    pub user_id: uuid::Uuid,
    pub parent_id: Option<uuid::Uuid>,
    pub title: String,
    pub task_description: String,
    pub created_at: DateTime<Utc>,
    pub status_name: Option<StatusTable>,
    pub type_name: Option<TypeTable>,
}

impl From<(Task, StatusTable, TypeTable)> for JoinedTask {
    fn from((task, status, t): (Task, StatusTable, TypeTable)) -> Self {
        Self {
            status_name: Some(status),
            type_name: Some(t),
            id: task.id,
            user_id: task.user_id,
            parent_id: task.parent_id,
            title: task.title,
            task_description: task.task_description,
            created_at: task.created_at,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TaskDescription {
    pub title: String,
    pub task_description: String,
    pub parent_id: Option<uuid::Uuid>,
    pub task_type: String,
    pub status_type: String,
}

#[derive(Debug, Insertable, Serialize, Deserialize, PartialEq)]
#[table_name = "tasks"]
pub struct TaskInfo {
    pub user_id: uuid::Uuid,
    pub parent_id: Option<uuid::Uuid>,
    pub title: String,
    pub task_description: String,
    pub status_name: uuid::Uuid,
    pub type_name: uuid::Uuid,
}

impl TaskInfo {
    pub fn with_owner_id(
        task: TaskDescription,
        status: StatusTable,
        t: TypeTable,
        owner_id: uuid::Uuid,
    ) -> Self {
        Self {
            user_id: owner_id,
            title: task.title,
            task_description: task.task_description,
            parent_id: task.parent_id,
            status_name: status.id,
            type_name: t.id,
        }
    }
}

impl Task {
    pub fn create_new_task(task: &TaskInfo, conn: &Connection) -> Result<uuid::Uuid, ServiceError> {
        Ok(diesel::insert_into(tasks)
            .values(task)
            .returning(id)
            .get_result(conn)?)
    }

    pub fn get_users_tasks(
        owner_id: uuid::Uuid,
        conn: &Connection,
    ) -> Result<Vec<Task>, ServiceError> {
        Ok(tasks.filter(user_id.eq(owner_id)).load::<Task>(conn)?)
    }

    pub fn get_users_tasks_with_join(
        owner_id: uuid::Uuid,
        conn: &Connection,
    ) -> Result<Vec<JoinedTask>, ServiceError> {
        Ok(tasks
            .inner_join(task_status_for_user::table)
            .inner_join(task_types_for_user::table)
            .filter(user_id.eq(owner_id))
            .load::<(Task, StatusTable, TypeTable)>(conn)?
            .into_iter()
            .map(|x| x.into())
            .collect())
    }

    pub fn get_children_tasks(
        owner_id: uuid::Uuid,
        parent: uuid::Uuid,
        conn: &Connection,
    ) -> Result<Vec<Task>, ServiceError> {
        Ok(tasks
            .filter(user_id.eq(owner_id))
            .filter(parent_id.eq(parent))
            .load::<Task>(conn)?)
    }

    pub fn get_task_by_id(task_id: uuid::Uuid, conn: &Connection) -> Result<Task, ServiceError> {
        Ok(tasks.filter(id.eq(task_id)).get_result(conn)?)
    }

    pub fn set_new_status(
        task_id: uuid::Uuid,
        new_status: StatusTable,
        conn: &Connection,
    ) -> Result<(), ServiceError> {
        diesel::update(tasks.filter(id.eq(task_id)))
            .set(status_name.eq(new_status.id))
            .execute(conn)?;
        Ok(())
    }
}
