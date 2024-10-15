use crate::model::error_model::PersistenceError;
use crate::model::response_model::{ListTableItemsResponse};
use crate::model::table_model::TableItem;
use actix_request_identifier::RequestId;
use log::{debug, error};
use mysql::prelude::*;
use mysql::{Pool};

pub fn get_table_items(pool: &Pool,
                       request_id: RequestId,
                       table_number: u32,
                       items_ids: Option<Vec<u32>>,
                       items_names: Option<Vec<String>>,
) -> Result<ListTableItemsResponse, PersistenceError> {
    let query = generate_query(table_number, items_ids, items_names);
    debug!("{request_id}; SQL Prepared for get table items");

    let mut conn = pool.get_conn().map_err(|_| PersistenceError::DBConnError)?;
    match conn.query_map(
        query,
        |(item_id, table_number, item_name, prepare_minutes, ordered_on)
        | TableItem { item_id, table_number, item_name, prepare_minutes, ordered_on }
    ) {
        Ok(table_items) => {
            debug!("{request_id}; SQL Executed successfully");
            Ok(generate_success_response(table_items))
        }
        Err(e) => {
            debug!("{request_id}; SQL Execution failed");
            error!("{:?}", e);
            Ok(generate_failed_response())
        }
    }
}




fn generate_failed_response() -> ListTableItemsResponse {
    ListTableItemsResponse {
        status: "failed".to_string(),
        message: "Could not find desired table items".to_string(),
        table_items: Vec::new(),
    }
}

fn generate_success_response(table_items: Vec<TableItem>) -> ListTableItemsResponse {
    ListTableItemsResponse {
        status: "success".to_string(),
        message: format!("Found {} table item(s)", table_items.len()),
        table_items,
    }
}

fn generate_query(table_number: u32, items_ids: Option<Vec<u32>>, items_names: Option<Vec<String>>) -> String {
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
    query
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