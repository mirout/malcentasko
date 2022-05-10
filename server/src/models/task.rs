use crate::{
    config::Connection,
    errors::ServiceError,
    schema::tasks::{self, dsl::*},
};
use chrono::{DateTime, Utc};
use diesel::prelude::*;

#[derive(Debug, Identifiable, Queryable, Serialize, Deserialize)]
pub struct Task {
    pub id: uuid::Uuid,
    pub owner_id: uuid::Uuid,
    pub parent_id: Option<uuid::Uuid>,
    pub title: String,
    pub task_description: String,
    pub created_at: DateTime<Utc>,
    pub done_at: Option<DateTime<Utc>>,
    pub is_done: bool,
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

    pub fn get_users_tasks(
        user_id: uuid::Uuid,
        conn: &Connection,
    ) -> Result<Vec<Task>, ServiceError> {
        Ok(tasks.filter(owner_id.eq(user_id)).load::<Task>(conn)?)
    }

    pub fn get_children_tasks(
        user_id: uuid::Uuid,
        parent: uuid::Uuid,
        conn: &Connection,
    ) -> Result<Vec<Task>, ServiceError> {
        Ok(tasks
            .filter(owner_id.eq(user_id))
            .filter(parent_id.eq(parent))
            .load::<Task>(conn)?)
    }

    pub fn get_task_by_id(task_id: uuid::Uuid, conn: &Connection) -> Result<Task, ServiceError> {
        Ok(tasks.filter(id.eq(task_id)).get_result(conn)?)
    }

    pub fn set_new_is_done_value(
        task_id: uuid::Uuid,
        new_value: bool,
        conn: &Connection,
    ) -> Result<(), ServiceError> {
        diesel::update(tasks.filter(id.eq(task_id)).filter(is_done.eq(!new_value)))
            .set((
                is_done.eq(new_value),
                if new_value {
                    done_at.eq(Some(chrono::Utc::now()))
                } else {
                    done_at.eq(None)
                },
            ))
            .execute(conn)?;

        Ok(())
    }
}
