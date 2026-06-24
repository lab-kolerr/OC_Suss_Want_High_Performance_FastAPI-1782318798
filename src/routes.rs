use actix_web::{web, HttpResponse, Responder};
use crate::models::*;

pub async fn create_ai_behavior(ai_behavior: web::Json<AIBehavior>) -> impl Responder {
    HttpResponse::Created().json(ai_behavior.into_inner())
}

pub async fn get_agents() -> impl Responder {
    HttpResponse::Ok().json(vec![]) // Placeholder for actual data
}

pub async fn create_alert(alert: web::Json<Alert>) -> impl Responder {
    HttpResponse::Created().json(alert.into_inner())
}

pub fn config_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/ai_behavior").route(web::post().to(create_ai_behavior))
            .service(web::resource("/agents").route(web::get().to(get_agents)))
            .service(web::resource("/alerts").route(web::post().to(create_alert)))
    );
}