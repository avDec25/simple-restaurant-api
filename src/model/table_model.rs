use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Serialize, Deserialize)]
pub struct TableItem {
    pub item_id: u32,
    pub table_number: u32,
    pub item_name: String,
    pub ordered_on: String,
    pub prepare_minutes: u32,
}

