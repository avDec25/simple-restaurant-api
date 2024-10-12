use actix_web::{HttpResponse, ResponseError};
use derive_more::{Display, Error, From};
use log::{error};
use serde::Serialize;

#[derive(Debug, Display, Error, From)]
pub enum PersistenceError {
    DBConnError,
    TransactionStartError,
    CommitError,
    RollbackError,
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
        }
    }
}
