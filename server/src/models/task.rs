use crate::schema::tasks::{self, dsl::*};
use diesel::prelude::*;

#[derive(Identifiable, Queryable, Serialize, Deserialize)]
pub struct Task {
    pub id: uuid::Uuid,
    pub owner_id: uuid::Uuid,
    pub parent_id: Option<uuid::Uuid>,
    pub title: String,
    pub task_description: String,
}