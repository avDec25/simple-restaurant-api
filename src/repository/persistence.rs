use crate::model::table_model::TableItem;
use crate::model::response_model::{AddItemsResponse, ListTableItemsResponse, RemoveTableItemResponse};
use chrono::Local;
use log::{error};
use mysql::Pool;
use mysql::prelude::*;
use rand::Rng;


pub fn add_items_to_table(
    pool: &Pool,
    table_number: u8,
    items_names: Vec<String>,
) -> AddItemsResponse {
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

    let mut conn = pool.get_conn().unwrap();
    conn.query_drop("START TRANSACTION").unwrap();  // Begin a transaction

    let query = format!(
        "INSERT INTO table_items (table_number, item_name, ordered_on, prepare_minutes) VALUES {}",
        values.join(", ")
    );

    let mut item_ids = Vec::new();
    match conn.query_drop(query) {
        Ok(_) => {
            let last_id = conn.query_first::<u8, _>("SELECT LAST_INSERT_ID()").unwrap_or(Some(0)).unwrap();
            for i in 0..values.len() {
                item_ids.push(last_id + i as u8);
            }
            conn.query_drop("COMMIT").unwrap();  // Commit the transaction
            let response = AddItemsResponse {
                status: "success".to_string(),
                message: format!("Added {} item(s) to table number {}", &items_names.len(), table_number),
                items_ids: item_ids,
            };
            response
        }
        Err(e) => {
            conn.query_drop("ROLLBACK").unwrap();  // Rollback the transaction on error
            error!("{:?}", e);
            AddItemsResponse {
                status: "failed".to_string(),
                message: format!("Could not add item(s) to table number {}", table_number),
                items_ids: item_ids,
            }
        }
    }
}

pub fn get_table_items(pool: &Pool,
                       table_number: u8,
                       items_ids: Option<Vec<u8>>,
                       items_names: Option<Vec<String>>,
) -> ListTableItemsResponse {
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
    match conn.query_map(query, |(item_id, table_number, item_name, prepare_minutes, ordered_on)| TableItem { item_id, table_number, item_name, prepare_minutes, ordered_on }) {
        Ok(table_items) => {
            ListTableItemsResponse {
                status: "success".to_string(),
                message: format!("Found {} table item(s)", table_items.len()),
                table_items,
            }
        }
        Err(e) => {
            error!("{:?}", e);
            ListTableItemsResponse {
                status: "failed".to_string(),
                message: "Could not find desired table items".to_string(),
                table_items: Vec::new(),
            }
        }
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


pub fn remove_table_item(
    pool: &Pool,
    item_id: u8,
) -> RemoveTableItemResponse {
    let query = format!(
        "DELETE FROM table_items WHERE item_id = '{}'",
        item_id
    );

    let mut conn = pool.get_conn().unwrap();
    conn.query_drop("START TRANSACTION").unwrap();

    match conn.query_drop(query) {
        Ok(_) => {
            let affected_rows = conn.affected_rows();
            conn.query_drop("COMMIT").unwrap();
            if affected_rows > 0 {
                RemoveTableItemResponse {
                    status: "success".to_string(),
                    message: format!("Removed table item with item_id {}", item_id),
                }
            } else {
                RemoveTableItemResponse {
                    status: "success".to_string(),
                    message: format!("No table item exists with item_id {}", item_id),
                }
            }
        }
        Err(e) => {
            conn.query_drop("ROLLBACK").unwrap();
            error!("{:?}", e);
            RemoveTableItemResponse {
                status: "failed".to_string(),
                message: "Could not remove the desired table item".to_string(),
            }
        }
    }
}