use actix_web::http::StatusCode;
use chrono::Local;
use derive_more::{Display, Error, From};
use mysql::{params, prelude::*};
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
    for name in items_name {
        if name.replace(' ', "").trim().is_empty() {
            continue;
        }
        let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

        pool.get_conn().unwrap().exec_drop(
            r"INSERT INTO table_items (
                    table_id, item_name, ordered_on, prepare_mins
                   ) VALUES (:table_id, :item_name, :ordered_on, :prepare_mins)",
            mysql::params! {
                "table_id" => table_number,
                "item_name" => name.clone(),
                "ordered_on" => timestamp,
                "prepare_mins" => rand::thread_rng().gen_range(5..16),
            },
        ).expect(format!(
            "Failed: Unable to add item {} to table_number {}",
            name, table_number).as_str()
        );
    }
}