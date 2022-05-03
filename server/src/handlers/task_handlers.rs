use actix_web::{get, post, web, HttpResponse, Responder};

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

#[get("/get_children_tasks")]
pub async fn get_children_tasks(
    owner: AuthorisedUser,
    web::Query(id): web::Query<uuid::Uuid>,
    pool: web::Data<Pool>,
) -> impl Responder {
    blocking_request!(pool, task_service::get_children_tasks[owner.0, id, &pool])
}
