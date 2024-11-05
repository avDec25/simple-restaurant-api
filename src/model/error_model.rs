use actix_web::{HttpResponse, ResponseError};
use derive_more::{Display, Error, From};
use log::{error};
use serde::Serialize;
use mysql::Error;

#[derive(Debug, Display, Error, From)]
pub enum PersistenceError {
    DBConnError,
    TransactionStartError,
    CommitError,
    RollbackError,
    ResourceNotFound,
    DBOpError,
}

#[derive(Debug, Display, Error, Serialize)]
pub struct PersistenceErrorResponse {
    message: String,
}

impl ResponseError for PersistenceError {
    fn error_response(&self) -> HttpResponse {
        match self {
            PersistenceError::DBConnError => {
                error!("Failed: Unable to acquire a connection from database connection pool");
                HttpResponse::InternalServerError().json(PersistenceErrorResponse {
                    message: "Failed: Error connecting with Database".to_string(),
                })
            }

            PersistenceError::TransactionStartError => {
                error!("Failed: Unable to Start Transaction");
                HttpResponse::InternalServerError().json(PersistenceErrorResponse {
                    message: "Failed: Database busy, Please try again later.".to_string(),
                })
            }

            PersistenceError::CommitError => {
                error!("Failed: Unable to Commit Transaction");
                HttpResponse::InternalServerError().json(PersistenceErrorResponse {
                    message: "Failed: Database busy, Please try again later.".to_string(),
                })
            }

            PersistenceError::RollbackError => {
                error!("Failed: Unable to Rollback Transaction");
                HttpResponse::InternalServerError().json(PersistenceErrorResponse {
                    message: "Failed: Database busy, Please report rollback failure".to_string(),
                })
            }

            PersistenceError::ResourceNotFound => {
                error!("Failed: Resource Not Found");
                HttpResponse::NotFound().json(PersistenceErrorResponse {
                    message: "Failed: No valid table items found".to_string(),
                })
            }

            PersistenceError::DBOpError => {
                error!("Failed: Unable to Execute SQL");
                HttpResponse::InternalServerError().json(PersistenceErrorResponse {
                    message: "Failed: SQL Execution failure".to_string(),
                })
            }
        }
    }
}


#[derive(Debug, Display, Error, From)]
pub enum MysqlValueError {
    MissingString,
    MissingInteger,
    MissingDatetime
}

pub fn generate_mysql_value_error(err_type: MysqlValueError, column_name: String) -> Error {
    Error::MySqlError(mysql::MySqlError {
        state: "failed".to_string(),
        code: match err_type {
            MysqlValueError::MissingString => 1,
            MysqlValueError::MissingInteger => 2,
            MysqlValueError::MissingDatetime => 3
        },
        message: format!("Error: Issue with value existing in column({column_name})"),
    })
}