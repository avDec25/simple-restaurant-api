use actix_web::http::StatusCode;
use chrono::Local;
use derive_more::{Display, Error, From};
use mysql::prelude::*;
use rand::Rng;

#[derive(Debug, Display, Error, From)]
pub enum PersistenceError {
    EmptyItemNames,

    // MysqlError(mysql::Error),
    //
    // Unknown,
}

impl actix_web::ResponseError for PersistenceError {
    fn status_code(&self) -> StatusCode {
        match self {
            PersistenceError::EmptyItemNames => StatusCode::BAD_REQUEST,

            // PersistenceError::MysqlError(_) | PersistenceError::Unknown => {
            //     StatusCode::INTERNAL_SERVER_ERROR
            // }
        }
    }
}

pub fn add_items_to_table(
    pool: &mysql::Pool,
    table_number: u8,
    items_name: Vec<String>,
) {
    let mut values = Vec::new();
    for name in items_name {
        if name.replace(' ', "").trim().is_empty() {
            continue;
        }
        let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        let prepare_mins = rand::thread_rng().gen_range(5..16);

        values.push(format!(
            "({}, '{}', '{}', {})",
            table_number,
            name,
            timestamp,
            prepare_mins
        ));
    }

    let query = format!(
        "INSERT INTO table_items (table_id, item_name, ordered_on, prepare_mins) VALUES {}",
        values.join(", ")
    );
    let mut conn = pool.get_conn().unwrap();
    conn.query_drop(query).expect("Failed to insert items");
}