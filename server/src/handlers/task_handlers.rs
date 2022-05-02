use actix_web::{post, web, HttpResponse, Responder};

use crate::{
    config::Pool,
    handlers::utils::unpack_result,
    models::{task::TaskDescription, user::AuthorisedUser},
    services::task_service,
};

#[post("/create_new_task")]
pub async fn create_new_task(
    owner: AuthorisedUser,
    task: web::Json<TaskDescription>,
    pool: web::Data<Pool>,
) -> impl Responder {
    let pool = pool.get().expect("Expected connections");
    let res =
        unpack_result(web::block(move || task_service::create_task(owner.0, task.0, &pool)).await);

    match res {
        Ok(val) => HttpResponse::Ok().body(serde_json::to_string(&val).unwrap()),
        Err(e) => e,
    }
}
