pub mod task_handlers;
pub mod user_handlers;

pub mod ping {
    use actix_web::{get, HttpResponse};

    #[get("/ping")]
    pub async fn ping() -> HttpResponse {
        HttpResponse::Ok().body("pong!".to_string())
    }
}

mod utils {
    use actix_web::{error::BlockingError, HttpResponse};

    use crate::errors::ServiceError;

    pub fn unpack_result<T>(
        result: Result<Result<T, ServiceError>, BlockingError>,
    ) -> Result<T, HttpResponse> {
        result
            .map_err(|_| HttpResponse::InternalServerError().finish())
            .and_then(|x| x.map_err(|e| e.into()))
            .map(|u| u.into())
    }
}
