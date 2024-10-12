use crate::model::table_model::TableItem;
use serde::Serialize;

#[derive(Serialize)]
pub struct AddItemsResponse {
    pub status: String,
    pub message: String,
    pub items_ids: Vec<u8>,
}

#[derive(Serialize)]
pub struct ListTableItemsResponse {
    pub status: String,
    pub message: String,
    pub table_items: Vec<TableItem>
}


#[derive(Serialize)]
pub struct RemoveTableItemResponse {
    pub status: String,
    pub message: String
}