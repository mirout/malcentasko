use actix_web::{error::BlockingError, HttpResponse};

use crate::errors::ServiceError;

pub mod auth {
    use actix_web::{post, web, HttpResponse, Responder};

    use crate::{
        config::Pool, handlers::utils::unpack_result, models::user::UserCredentials,
        services::user_service,
    };

    #[post("/signup")]
    pub async fn signup(user: web::Json<UserCredentials>, pool: web::Data<Pool>) -> impl Responder {
        let pool = pool.get().expect("Expected connections");
        let res = unpack_result(web::block(move || user_service::signup(user.0, &pool)).await);

        match res {
            Ok(val) => HttpResponse::Ok().body(serde_json::to_string(&val).unwrap()),
            Err(e) => e,
        }
    }

    #[post("/login")]
    pub async fn login(user: web::Json<UserCredentials>, pool: web::Data<Pool>) -> impl Responder {
        let pool = pool.get().expect("Expected connections");
        let res = unpack_result(web::block(move || user_service::login(user.0, &pool)).await);

        match res {
            Ok(val) => HttpResponse::Ok().body(serde_json::to_string(&val).unwrap()),
            Err(e) => e,
        }
    }
}
