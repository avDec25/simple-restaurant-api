use actix_web::{get, HttpResponse, Responder};
use chrono::Local;

#[get("/health_check")]
pub async fn health_check() -> impl Responder {
    let date = Local::now();
    HttpResponse::Ok().body(format!("Health OK at {}", date.format("%Y-%m-%d][%H:%M:%S")))
}