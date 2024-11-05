use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AddItemsToTableRequest {
    pub items_names: Vec<String>,
}
