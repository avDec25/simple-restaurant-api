use crate::model::error_model::PersistenceError;
use crate::model::response_model::ListTableItemsResponse;
use crate::model::table_model::TableItem;
use actix_request_identifier::RequestId;
use log::{debug, error, warn};
use mysql::prelude::*;
use mysql::{from_value, Pool, Value, Row};
use chrono::{NaiveDate, NaiveDateTime, NaiveTime};

pub fn get_table_items(
    pool: &Pool,
    request_id: RequestId,
    table_number: u32,
    items_ids: Option<Vec<u32>>,
    items_names: Option<Vec<String>>,
) -> Result<ListTableItemsResponse, PersistenceError> {
    let (query, params) = generate_query_and_params(table_number, items_ids, items_names);
    debug!("{request_id}; SQL Prepared for get table items");

    let mut conn = pool.get_conn().map_err(|_| PersistenceError::DBConnError)?;
    let result = match conn.exec_iter(query, params) {
        Ok(result) => {
            let table_items: Vec<TableItem> = result
                .map(|row| row_to_table_item(row))
                .filter_map(Result::ok)
                .collect();

            if table_items.is_empty() {
                warn!("{request_id}; No valid table items found");
                Ok(generate_empty_response())
            } else {
                debug!("{request_id}; SQL Executed successfully");
                Ok(generate_success_response(table_items))
            }
        }
        Err(e) => {
            error!("{request_id}; SQL Execution failed: {:?}", e);
            Ok(generate_failed_response())
        }
    }; result
}


fn generate_failed_response() -> ListTableItemsResponse {
    ListTableItemsResponse {
        status: "failed".to_string(),
        message: "Could not fetch table items".to_string(),
        table_items: Vec::new(),
    }
}

fn generate_empty_response() -> ListTableItemsResponse {
    ListTableItemsResponse {
        status: "success".to_string(),
        message: "No table items found".to_string(),
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

fn generate_query_and_params(table_number: u32, items_ids: Option<Vec<u32>>,
                             items_names: Option<Vec<String>>
) -> (String, Vec<Value>) {
    let mut query = String::from("SELECT * FROM table_items WHERE table_number = ?");
    let mut params: Vec<Value> = vec![Value::from(table_number)];
    let mut conditions = Vec::new();

    if let Some(ref items_ids) = items_ids {
        if !items_ids.is_empty() {
            conditions.push(format!("item_id IN ({})", vec!["?"; items_ids.len()].join(",")));
            params.extend(items_ids.iter().map(|&id| Value::from(id)));
        }
    }

    if let Some(ref items_names) = items_names {
        if !items_names.is_empty() {
            conditions.push(format!("item_name IN ({})", vec!["?"; items_names.len()].join(",")));
            params.extend(items_names.iter().map(|name| Value::from(name.clone())));
        }
    }

    if !conditions.is_empty() {
        query.push_str(" AND ");
        if conditions.len() == 2 {
            query.push_str(&format!("( {} )", conditions.join(" OR ")));
        } else {
            query.push_str(&conditions.join(" OR "));
        }
    }
    (query, params)
}

fn row_to_table_item(row: Result<Row, mysql::Error>) -> Result<TableItem, mysql::Error> {
    let row = row?;
    let item_id: u32 = row.get(0)
        .ok_or_else(|| mysql::Error::MySqlError(mysql::MySqlError {
            state: "Error: Conversion to u32 failed".to_string(),
            code: 0,
            message: "Missing item_id".into(),
        }))?;
    let table_number: u32 = row.get(1)
        .ok_or_else(|| mysql::Error::MySqlError(mysql::MySqlError {
            state: "Error: Conversion to u32 failed".to_string(),
            code: 0,
            message: "Missing table_number".into(),
        }))?;
    let item_name: String = match row.get(2) {
        Some(Value::Bytes(bytes)) => String::from_utf8_lossy(&*bytes).into_owned(),
        Some(other) => from_value(other),
        None => return Err(mysql::Error::MySqlError(mysql::MySqlError {
            state: "Error: Conversion to u32 failed".to_string(),
            code: 0,
            message: "Missing item_name".into(),
        })),
    };
    let prepare_minutes: u32 = row.get(3)
        .ok_or_else(|| mysql::Error::MySqlError(mysql::MySqlError {
            state: "Error: Conversion to u32 failed".to_string(),
            code: 0,
            message: "Missing prepare_minutes".into(),
        }))?;
    let ordered_on: String = match row.get(4) {
        Some(Value::Date(year, month, day, hour, minute, second, _micro_second)) => {
            NaiveDateTime::new(
                NaiveDate::from_ymd_opt(year as i32, month as u32, day as u32).unwrap(),
                NaiveTime::from_hms_opt(hour as u32, minute as u32, second as u32).unwrap()
            ).format("%Y-%m-%d %H:%M:%S").to_string()
        },
        Some(other) => from_value(other),
        None => return Err(mysql::Error::MySqlError(mysql::MySqlError {
            state: "Error: Conversion to String failed".to_string(),
            code: 1,
            message: "Missing ordered_on".into(),
        })),
    };

    Ok(TableItem {
        item_id,
        table_number,
        item_name,
        prepare_minutes,
        ordered_on,
    })
}