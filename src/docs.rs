use actix_web::{web, HttpResponse};

pub async fn api_docs() -> HttpResponse {
    HttpResponse::Ok().body("API Documentation")
}

pub fn config_docs_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/docs").route(web::get().to(api_docs)));
}