use actix_web::web;
use diesel::{r2d2::ConnectionManager, PgConnection};
use diesel_migrations::embed_migrations;

use crate::handlers;

pub fn config_from_env(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(handlers::ping::ping)
            .service(
                web::scope("/auth")
                    .service(handlers::user_handlers::auth::signup)
                    .service(handlers::user_handlers::auth::login),
            )
            .service(
                web::scope("/tasks")
                    .service(handlers::task_handlers::create_new_task)
                    .service(handlers::task_handlers::get_users_tasks)
                    .service(handlers::task_handlers::update_task_status),
            ),
    );
}

embed_migrations!();

pub type Connection = PgConnection;
pub type Pool = r2d2::Pool<ConnectionManager<Connection>>;

pub fn get_pool(url: &str) -> Pool {
    let manager = ConnectionManager::<Connection>::new(url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool");

    embedded_migrations::run(&pool.get().expect("Failed to migrate")).expect("Failed to migrate");
    return pool;
}
