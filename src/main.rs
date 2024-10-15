use crate::api::health_check_api::health_check;
use crate::api::table_api::{add_items, get_items, remove_item};
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use log::info;
use mysql::Pool;
use std::env;
use actix_request_identifier::RequestIdentifier;

mod api;
mod model;
mod repository;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // setting up environment
    dotenvy::dotenv().ok();
    // setting up logging
    env_logger::init_from_env(env_logger::Env::new().default_filter_or(
        env::var("LOGGING_LEVEL")
            .expect("Please Check LOGGING_LEVEL env var")
    ));

    // setting up database
    let db_host = env::var("MYSQL_HOST").expect("Please check MYSQL_HOST env var");
    let db_port = env::var("MYSQL_PORT").expect("Please check MYSQL_PORT env var");
    let db_port:u16 = db_port.parse().expect("Please check PORT env var");
    let db_name = env::var("MYSQL_DBNAME")
        .expect("Please check MYSQL_DBNAME env var");
    let db_user = env::var("MYSQL_USER")
        .expect("Please check MYSQL_USER env var");
    let db_password = env::var("MYSQL_PASSWORD")
        .expect("Please check MYSQL_PASSWORD env var");

    let builder = mysql::OptsBuilder::new()
        .ip_or_hostname(Some(db_host))
        .tcp_port(db_port)
        .db_name(Some(db_name))
        .user(Some(db_user))
        .pass(Some(db_password));

    let pool = Pool::new(builder).unwrap();
    let shared_data = web::Data::new(pool);
    info!("Completed building Mysql Connection Pool");

    // setting up application
    let concurrent_workers = env::var("CONCURRENT_WORKERS")
        .expect("Please check CONCURRENT_WORKERS env var");
    let concurrent_workers = concurrent_workers.parse()
        .expect("Please check CONCURRENT_WORKERS env var");
    let app_host = env::var("APP_HOST").expect("Please check APP_HOST env var");
    let app_port = env::var("APP_PORT").expect("Please check APP_PORT env var");
    let app_port:u16 = app_port.parse().expect("Please check APP_PORT env var");
    let bind_address = format!("{}:{}", app_host, app_port);

    info!("Starting simple-restaurant-api server at http://localhost:8080");
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(RequestIdentifier::with_uuid())
            .app_data(shared_data.clone())
            .service(health_check)
            .service(add_items)
            .service(get_items)
            .service(remove_item)
    }).bind(bind_address)?
        .workers(concurrent_workers)
        .run()
        .await
}
