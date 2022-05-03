use actix_web::{error::BlockingError, HttpResponse};

use crate::errors::ServiceError;

pub mod auth {
    use actix_web::{post, web, HttpResponse, Responder};

    use crate::{
        blocking_request, config::Pool, handlers::utils::unpack_result,
        models::user::UserCredentials, services::user_service,
    };

    #[post("/signup")]
    pub async fn signup(user: web::Json<UserCredentials>, pool: web::Data<Pool>) -> impl Responder {
        blocking_request!(pool, user_service::signup[user.0, &pool])
    }

    #[post("/login")]
    pub async fn login(user: web::Json<UserCredentials>, pool: web::Data<Pool>) -> impl Responder {
        blocking_request!(pool, user_service::login[user.0, &pool])
    }
}
