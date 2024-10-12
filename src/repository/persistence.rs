use crate::model::table_model::TableItem;
use actix_web::http::StatusCode;
use chrono::Local;
use derive_more::{Display, Error, From};
use log::info;
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
    items_names: Vec<String>,
) {
    let mut values = Vec::new();
    for item_name in items_names {
        if item_name.replace(' ', "").trim().is_empty() {
            continue;
        }
        let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        let prepare_minutes = rand::thread_rng().gen_range(5..16);

        values.push(format!(
            "({}, '{}', '{}', {})",
            table_number,
            item_name,
            timestamp,
            prepare_minutes
        ));
    }

    let query = format!(
        "INSERT INTO table_items (table_number, item_name, ordered_on, prepare_minutes) VALUES {}",
        values.join(", ")
    );
    let mut conn = pool.get_conn().unwrap();
    conn.query_drop(query).expect("Failed to insert table items");
}

pub fn get_table_items(pool: &mysql::Pool,
                       table_number: u8,
                       items_ids: Option<Vec<u8>>,
                       items_names: Option<Vec<String>>
) {
    let mut query = format!(
        "SELECT * FROM table_items WHERE table_number = '{}'",
        table_number
    );

    let conditions = compute_conditions(items_ids, items_names);
    if !conditions.is_empty() {
        query.push_str(" AND ");
        if conditions.len() == 2 {
            query.push_str(&format!("( {} )", conditions.join(" OR ")));
        } else {
            query.push_str(&conditions.join(" OR "));
        }
    }

    let mut conn = pool.get_conn().unwrap();
    let results = conn.query_map(query,
                                 |(
                                      item_id,
                                      table_number,
                                      item_name,
                                      prepare_minutes,
                                      ordered_on
                                  )| TableItem {
                                     item_id,
                                     table_number,
                                     item_name,
                                     prepare_minutes,
                                     ordered_on,
                                 }).unwrap();

    for row in results {
        println!("{:?}", row);
    }
}

fn compute_conditions(items_ids: Option<Vec<u8>>,
                      items_names: Option<Vec<String>>,
) -> Vec<String> {
    let mut conditions = Vec::new();

    if let Some(ref items_ids) = items_ids {
        if !items_ids.is_empty() {
            let ids: Vec<String> = items_ids.iter().map(|id| id.to_string()).collect();
            let ids_str = ids.join(", ");
            conditions.push(format!("item_id IN ({})", ids_str));
        }
    }

    if let Some(ref items_names) = items_names {
        if !items_names.is_empty() {
            let names: Vec<String> = items_names.iter().map(|name| format!("'{}'", name)).collect();
            let names_str = names.join(", ");
            conditions.push(format!("item_name IN ({})", names_str));
        }
    }

    conditions
}


pub fn remove_table_item(pool: &mysql::Pool, item_id: u8) {
    let query = format!(
        "DELETE FROM table_items WHERE item_id = '{}'",
        item_id
    );
    let mut conn = pool.get_conn().unwrap();
    conn.query_drop(query).expect("Delete table item");
    info!("Deleted item with id {item_id}")
}