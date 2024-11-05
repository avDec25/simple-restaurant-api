use serde::Serialize;

#[derive(Serialize)]
pub struct AddItemsResponse {
    pub status: String,
    pub message: String,
    pub items_ids: Vec<u32>,
}


#[derive(Serialize)]
pub struct RemoveTableItemResponse {
    pub status: String,
    pub message: String
}