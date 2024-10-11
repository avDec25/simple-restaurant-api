use serde::Deserialize;

// #[derive(Debug, Serialize, Deserialize)]
// pub struct TableItem {
//     pub item_id: u8,
//     pub table_id: u8,
//     pub item_name: String,
//     pub ordered_on: String,
//     pub prepare_minutes: u8,
// }

#[derive(Debug, Deserialize)]
pub struct AddItemsToTableRequest {
    pub table_num: u8,
    pub items_name: Vec<String>,
}

// #[derive(Debug, Deserialize)]
// pub struct GetAllTableItemsRequest {
//     pub table_num: u8,
// }
//
// #[derive(Debug, Deserialize)]
// pub struct GetPendingTableItemsRequest {
//     pub table_num: u8,
// }
//
// #[derive(Debug, Deserialize)]
// pub struct GetSpecifiedTableItemsRequest {
//     pub table_num: u8,
//     pub item_ids: Vec<u8>,
// }
//
// #[derive(Debug, Deserialize)]
// pub struct RemoveTableItemsRequest {
//     pub table_num: u8,
//     pub item_ids: Vec<u8>,
// }