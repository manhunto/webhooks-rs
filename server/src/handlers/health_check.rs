use actix_web::HttpResponse;

pub async fn health_check() -> HttpResponse {
    HttpResponse::NoContent().finish()
}
