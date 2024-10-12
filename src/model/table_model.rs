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

