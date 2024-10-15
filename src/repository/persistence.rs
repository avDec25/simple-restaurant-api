use actix_request_identifier::RequestId;
use crate::model::error_model::PersistenceError;
use crate::model::response_model::{AddItemsResponse, ListTableItemsResponse, RemoveTableItemResponse};
use crate::model::table_model::TableItem;
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
    let query = format!(
        "INSERT INTO table_items (table_number, item_name, ordered_on, prepare_minutes) VALUES {}",
        values.join(", ")
    );
    debug!("{request_id}; SQL Prepared for adding items to table");

    let mut conn = match pool.get_conn() {
        Ok(conn) => {conn}
        Err(_) => { return Err(PersistenceError::DBConnError) }
    };

    debug!("{request_id}; Starting transaction");
    match conn.query_drop("START TRANSACTION") {
        Ok(_) => {}
        Err(_) => {return Err(PersistenceError::TransactionStartError)}
    }

    let mut item_ids = Vec::new();
    match conn.query_drop(query) {
        Ok(_) => {
            let last_id = conn.query_first::<u32, _>("SELECT LAST_INSERT_ID()").unwrap_or(Some(0)).expect("Error: Unable to acquire the last inserted id");
            for i in 0..values.len() {
                item_ids.push(last_id + i as u32);
            }
            match conn.query_drop("COMMIT") {
                Ok(_) => {}
                Err(_) => {return Err(PersistenceError::CommitError)}
            }
            debug!("{request_id}; Transaction Completed");
            let response = AddItemsResponse {
                status: "success".to_string(),
                message: format!("Added {} item(s) to table number {}", &items_names.len(), table_number),
                items_ids: item_ids,
            };
            Ok(response)
        }
        Err(e) => {
            match conn.query_drop("ROLLBACK") {
                Ok(_) => {}
                Err(_) => {return Err(PersistenceError::RollbackError)}
            }
            debug!("{request_id}; Transaction could not be completed; Rolled back");
            error!("{:?}", e);
            Ok(AddItemsResponse {
                status: "failed".to_string(),
                message: format!("Could not add item(s) to table number {}", table_number),
                items_ids: item_ids,
            })
        }
    }
}


pub fn get_table_items(pool: &Pool,
                       request_id: RequestId,
                       table_number: u32,
                       items_ids: Option<Vec<u32>>,
                       items_names: Option<Vec<String>>,
) -> Result<ListTableItemsResponse, PersistenceError> {
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
    debug!("{request_id}; SQL Prepared for get table items");

    let mut conn = match pool.get_conn() {
        Ok(conn) => {conn},
        Err(_) => { return Err(PersistenceError::DBConnError) }
    };
    match conn.query_map(query, |(item_id, table_number, item_name, prepare_minutes, ordered_on)| TableItem { item_id, table_number, item_name, prepare_minutes, ordered_on }) {
        Ok(table_items) => {
            debug!("{request_id}; SQL Executed successfully");
            Ok(ListTableItemsResponse {
                status: "success".to_string(),
                message: format!("Found {} table item(s)", table_items.len()),
                table_items,
            })
        }
        Err(e) => {
            debug!("{request_id}; SQL Execution failed");
            error!("{:?}", e);
            Ok(ListTableItemsResponse {
                status: "failed".to_string(),
                message: "Could not find desired table items".to_string(),
                table_items: Vec::new(),
            })
        }
    }
}

fn compute_conditions(items_ids: Option<Vec<u32>>,
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


pub fn remove_table_item(
    pool: &Pool,
    request_id: RequestId,
    item_id: u32,
) -> Result<RemoveTableItemResponse, PersistenceError> {
    let query = format!(
        "DELETE FROM table_items WHERE item_id = '{}'",
        item_id
    );
    debug!("{request_id}; SQL Prepared for Item Delete operation");

    let mut conn = match pool.get_conn() {
        Ok(conn) => {conn},
        Err(_) => { return Err(PersistenceError::DBConnError) }
    };
    debug!("{request_id}; Starting transaction");
    match conn.query_drop("START TRANSACTION") {
        Ok(_) => {}
        Err(_) => {return Err(PersistenceError::TransactionStartError)}
    }

    match conn.query_drop(query) {
        Ok(_) => {
            let affected_rows = conn.affected_rows();
            match conn.query_drop("COMMIT") {
                Ok(_) => {}
                Err(_) => {return Err(PersistenceError::CommitError)}
            }
            debug!("{request_id}; Completed transaction");
            if affected_rows > 0 {
                Ok(RemoveTableItemResponse {
                    status: "success".to_string(),
                    message: format!("Removed table item with item_id {}", item_id),
                })
            } else {
                Ok(RemoveTableItemResponse {
                    status: "success".to_string(),
                    message: format!("No table item exists with item_id {}", item_id),
                })
            }
        }
        Err(e) => {
            match conn.query_drop("ROLLBACK") {
                Ok(_) => {}
                Err(_) => {return Err(PersistenceError::RollbackError)}
            }
            debug!("{request_id}; Transaction could not be completed; Rolled back");
            error!("{:?}", e);
            Ok(RemoveTableItemResponse {
                status: "failed".to_string(),
                message: "Could not remove the desired table item".to_string(),
            })
        }
    }
}