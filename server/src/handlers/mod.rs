pub mod ping {
    use actix_web::{get, HttpResponse};

    #[get("/ping")]
    pub async fn ping() -> HttpResponse {
        HttpResponse::Ok().body("pong!".to_string())
    }
}