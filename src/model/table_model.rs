use serde::Serialize;
use serde::Deserialize;

#[derive(Debug, Serialize, Deserialize)]
pub struct TableItem {
    pub item_id: u8,
    pub table_number: u8,
    pub item_name: String,
    pub ordered_on: String,
    pub prepare_minutes: u8,
}

#[derive(Debug, Deserialize)]
pub struct AddItemsToTableRequest {
    pub table_number: u8,
    pub items_names: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct ListTableItemsRequest {
    pub table_number: u8,
    pub items_ids: Option<Vec<u8>>,
    pub items_names: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
pub struct RemoveTableItemRequest {
    pub item_id: u8
}