use std::env;
use actix_web::{web, App, HttpServer};
use actix_web::middleware::Logger;
use log::info;
use mysql::Pool;
use crate::api::health_check_api::health_check;
use crate::api::table_api::add_items;

mod api;
mod model;
mod repository;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    info!("Building mysql connection pool");

    let db_host = env::var("MYSQL_HOST").expect("HOST is not set in .env file");
    let db_port = env::var("MYSQL_PORT").expect("PORT is not set in .env file");
    let db_port = db_port.parse().unwrap();
    let db_name = env::var("MYSQL_DBNAME").expect("DBNAME is not set in .env file");
    let db_user = env::var("MYSQL_USER").expect("USER is not set in .env file");
    let db_password = env::var("MYSQL_PASSWORD").expect("PASSWORD is not set in .env file");

    let builder = mysql::OptsBuilder::new()
        .ip_or_hostname(Some(db_host))
        .tcp_port(db_port)
        .db_name(Some(db_name))
        .user(Some(db_user))
        .pass(Some(db_password));

    let pool = Pool::new(builder).unwrap();
    let shared_data = web::Data::new(pool);

    log::info!("Starting HTTP server at http://localhost:8080");
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(shared_data.clone())
            .service(health_check)
            .service(add_items)
    }).bind("127.0.0.1:8080")?
        .workers(2)
        .run()
        .await
}
