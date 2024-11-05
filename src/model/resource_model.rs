use serde::Deserialize;
use serde::Serialize;
use hateoas::{HttpMethod, RelLink, RelLinkCollection};

#[derive(Serialize, Deserialize)]
pub struct TableItem {
    pub item_id: u32,
    pub table_number: u32,
    pub item_name: String,
    pub ordered_on: String,
    pub prepare_minutes: u32,
}


#[derive(Serialize, Deserialize)]
pub struct TableItemResource {
    pub item_id: u32,
    pub table_number: u32,
    pub item_name: String,
    pub ordered_on: String,
    pub prepare_minutes: u32,
    pub _links: RelLinkCollection,
}

impl TableItemResource {
    pub fn new(item_id: u32, table_number: u32, item_name: String, ordered_on: String, prepare_minutes: u32) -> Self {
        let mut resource = Self {
            item_id, table_number, item_name, ordered_on, prepare_minutes,
            _links: RelLinkCollection::new(vec![])
        };
        let self_link = format!("/tables/{}/items/{}", table_number, item_id);
        resource.add_link("self", &self_link, HttpMethod::Get);
        resource.add_link("delete", &self_link, HttpMethod::Delete);

        let table_link = format!("/tables/{}", table_number);
        resource.add_link("table", &table_link, HttpMethod::Get);

        resource
    }

    fn add_link(&mut self, rel: &str, href: &str, method: HttpMethod) {
        self._links.add(
            RelLink::new(rel, href, method)
        );
    }
}


#[derive(Serialize, Deserialize)]
pub struct TableResource {
    pub table_number: u32,
    pub items: Vec<TableItemResource>,
    pub _links: RelLinkCollection,
}

impl TableResource {
    pub(crate) fn new(table_number: u32, items: Vec<TableItemResource>) -> Self {
        let mut resource = Self {
            table_number,
            items,
            _links: RelLinkCollection::new(vec![]),
        };
        let self_link = format!("/tables/{table_number}");
        resource.add_link("self", self_link.as_str(), HttpMethod::Get);

        let add_item_link = format!("/tables/{table_number}");
        resource.add_link("add_items", add_item_link.as_str(), HttpMethod::Put);

        resource
    }
    fn add_link(&mut self, rel: &str, href: &str, method: HttpMethod) {
        self._links.add(
            RelLink::new(rel, href, method)
        );
    }
}