use serde::Deserialize;

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
    pub item_id: u8,
}