use crate::model::error_model::PersistenceError;
use crate::model::response_model::AddItemsResponse;
use actix_request_identifier::RequestId;
use chrono::Local;
use log::{debug, error};
use mysql::prelude::*;
use mysql::Pool;
use rand::Rng;

pub fn add_items_to_table(
    pool: &Pool,
    request_id: RequestId,
    table_number: u32,
    items_names: Vec<String>,
) -> Result<AddItemsResponse, PersistenceError> {
    let records: Vec<String> = generate_table_item_records(table_number, items_names);
    let query = generate_query(&records);
    debug!("{request_id}; SQL Prepared for adding items to table");

    let mut conn = pool.get_conn().map_err(|_| PersistenceError::DBConnError)?;
    debug!("{request_id}; Starting transaction");
    conn.query_drop("START TRANSACTION").map_err(|_| PersistenceError::TransactionStartError)?;

    match conn.query_drop(query) {
        Ok(_) => {
            let last_id = conn.query_first::<u32, _>("SELECT LAST_INSERT_ID()")
                .unwrap_or(Some(0)).expect("Error: Unable to acquire the last inserted id");
            let item_ids: Vec<u32> = (last_id..last_id + records.len() as u32).collect();
            conn.query_drop("COMMIT").map_err(|_| PersistenceError::CommitError)?;
            debug!("{request_id}; Transaction Completed");
            Ok(generate_success_response(table_number, records, item_ids))
        }
        Err(e) => {
            conn.query_drop("ROLLBACK").map_err(|_| PersistenceError::RollbackError)?;
            debug!("{request_id}; Transaction could not be completed; Rolled back");
            error!("{:?}", e);
            Ok(generate_failed_response(table_number))
        }
    }
}


fn generate_failed_response(table_number: u32) -> AddItemsResponse {
    AddItemsResponse {
        status: "failed".to_string(),
        message: format!("Could not add item(s) to table number {}", table_number),
        items_ids: [].to_vec(),
    }
}

fn generate_success_response(table_number: u32, values: Vec<String>, item_ids: Vec<u32>) -> AddItemsResponse {
    AddItemsResponse {
        status: "success".to_string(),
        message: format!("Added {} item(s) to table number {}", values.len(), table_number),
        items_ids: item_ids,
    }
}

fn generate_query(records: &Vec<String>) -> String {
    format!(
        "INSERT INTO table_items (table_number, item_name, ordered_on, prepare_minutes) VALUES {}",
        records.join(", ")
    )
}

fn generate_table_item_records(table_number: u32, items_names: Vec<String>) -> Vec<String> {
    let mut values = Vec::new();
    for item_name in &items_names {
        if item_name.replace(' ', "").trim().is_empty() {
            continue;
        }
        let ordered_on = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        let prepare_minutes = rand::thread_rng().gen_range(5..16);

        values.push(format!(
            "({}, '{}', '{}', {})",
            table_number,
            item_name,
            ordered_on,
            prepare_minutes
        ));
    }
    values
}
