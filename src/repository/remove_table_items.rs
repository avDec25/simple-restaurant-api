use crate::model::error_model::PersistenceError;
use crate::model::response_model::{RemoveTableItemResponse};
use actix_request_identifier::RequestId;
use log::{debug, error};
use mysql::prelude::*;
use mysql::{Pool};


pub fn remove_table_item(
    pool: &Pool,
    request_id: RequestId,
    item_id: u32,
) -> Result<RemoveTableItemResponse, PersistenceError> {
    let query = generate_query();
    debug!("{request_id}; SQL Prepared for Item Delete operation");

    let mut conn = pool.get_conn().map_err(|_| PersistenceError::DBConnError)?;

    debug!("{request_id}; Starting transaction");
    conn.query_drop("START TRANSACTION").map_err(|_| PersistenceError::TransactionStartError)?;

    match conn.exec_drop(query, (item_id,)) {
        Ok(_) => {
            let affected_rows = conn.affected_rows();
            conn.query_drop("COMMIT").map_err(|_| PersistenceError::CommitError)?;
            debug!("{request_id}; Completed transaction");
            if affected_rows > 0 {
                Ok(generate_success_response(item_id))
            } else {
                Ok(generate_absent_response(item_id))
            }
        }
        Err(e) => {
            conn.query_drop("ROLLBACK").map_err(|_| PersistenceError::RollbackError)?;
            debug!("{request_id}; Transaction could not be completed; Rolled back");
            error!("{:?}", e);
            Ok(generate_failed_response())
        }
    }
}

fn generate_absent_response(item_id: u32) -> RemoveTableItemResponse {
    RemoveTableItemResponse {
        status: "success".to_string(),
        message: format!("No table item exists with item_id {}", item_id),
    }
}

fn generate_failed_response() -> RemoveTableItemResponse {
    RemoveTableItemResponse {
        status: "failed".to_string(),
        message: "Could not remove the desired table item".to_string(),
    }
}

fn generate_success_response(item_id: u32) -> RemoveTableItemResponse {
    RemoveTableItemResponse {
        status: "success".to_string(),
        message: format!("Removed table item with item_id {}", item_id),
    }
}

fn generate_query() -> String {
    "DELETE FROM table_items WHERE item_id = ?".to_string()
}