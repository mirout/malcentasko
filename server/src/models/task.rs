use crate::{
    config::Connection,
    errors::ServiceError,
    schema::tasks::{self, dsl::*},
};
use diesel::prelude::*;

#[derive(Debug, Identifiable, Queryable, Serialize, Deserialize)]
pub struct Task {
    pub id: uuid::Uuid,
    pub owner_id: uuid::Uuid,
    pub parent_id: Option<uuid::Uuid>,
    pub title: String,
    pub task_description: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TaskDescription {
    pub title: String,
    pub task_description: String,
    pub parent_id: Option<uuid::Uuid>,
}

#[derive(Debug, Insertable, Serialize, Deserialize)]
#[table_name = "tasks"]
pub struct TaskInfo {
    pub owner_id: uuid::Uuid,
    pub parent_id: Option<uuid::Uuid>,
    pub title: String,
    pub task_description: String,
}

impl TaskInfo {
    pub fn with_owner_id(task: TaskDescription, user_id: uuid::Uuid) -> Self {
        Self {
            owner_id: user_id,
            title: task.title,
            task_description: task.task_description,
            parent_id: task.parent_id,
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
}
