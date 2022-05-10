use actix_web::{get, post, web, HttpResponse, Responder};
use diesel::deserialize;
use serde::{de, Deserialize, Deserializer};

use crate::{
    config::Pool,
    handlers::utils::*,
    models::{task::TaskDescription, user::AuthorisedUser},
    services::task_service,
};

use crate::blocking_request;

#[post("/create_new_task")]
pub async fn create_new_task(
    owner: AuthorisedUser,
    task: web::Json<TaskDescription>,
    pool: web::Data<Pool>,
) -> impl Responder {
    blocking_request!(pool, task_service::create_task[owner.0, task.0, &pool])
}

#[get("/get_users_tasks")]
pub async fn get_users_tasks(owner: AuthorisedUser, pool: web::Data<Pool>) -> impl Responder {
    blocking_request!(pool, task_service::get_users_tasks[owner.0, &pool])
}

#[derive(Deserialize, Serialize)]
pub struct TaskId {
    pub id: uuid::Uuid,
}

#[derive(Deserialize, Serialize)]
pub struct BooleanValue {
    #[serde(deserialize_with = "bool_from_str")]
    pub new_value: bool,
}

#[derive(Deserialize, Serialize)]
pub struct UpdateTaskStatus {
    #[serde(flatten)]
    pub id: TaskId,
    #[serde(flatten)]
    pub val: BooleanValue,
}

fn bool_from_str<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    use std::str::FromStr;
    let s = String::deserialize(deserializer)?;
    bool::from_str(&s).map_err(de::Error::custom)
}

#[get("/get_children_tasks")]
pub async fn get_children_tasks(
    owner: AuthorisedUser,
    web::Query(id): web::Query<TaskId>,
    pool: web::Data<Pool>,
) -> impl Responder {
    blocking_request!(pool, task_service::get_children_tasks[owner.0, id.id, &pool])
}

#[post("/update_task_status")]
pub async fn update_task_status(
    _: AuthorisedUser,
    web::Query(UpdateTaskStatus { id, val }): web::Query<UpdateTaskStatus>,
    pool: web::Data<Pool>,
) -> impl Responder {
    blocking_request!(pool, task_service::update_task_status[id.id, val.new_value, &pool])
}
