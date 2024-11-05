use actix_request_identifier::RequestId;
use crate::model::request_model::{
    AddItemsToTableRequest
};
use crate::repository::remove_table_items::remove_table_item;
use actix_web::{delete, get, post, web, HttpResponse, Responder};
use mysql::Pool;
use crate::model::error_model::PersistenceError;
use crate::model::resource_model::{TableItemResource, TableResource};
use crate::repository::persist_table_items::add_items_to_table;
use crate::repository::fetch_table_items::get_table_items;


#[post("/tables/{table_number}/items")]
pub(crate) async fn add_items(
    path: web::Path<u32>,
    web::Json(request): web::Json<AddItemsToTableRequest>,
    data: web::Data<Pool>,
    request_id: RequestId,
) -> actix_web::Result<impl Responder> {
    let table_number = path.into_inner();

    let response = web::block(move ||
        add_items_to_table(
            &data, request_id,
            table_number,
            request.items_names,
        )).await??;

    Ok(HttpResponse::Created().json(response))
}


#[get("/tables/{table_number}/items")]
pub(crate) async fn get_all_items(
    path: web::Path<u32>,
    data: web::Data<Pool>,
    request_id: RequestId,
) -> actix_web::Result<impl Responder> {
    let table_number = path.into_inner();

    let items = web::block(move ||
        get_table_items(&data, request_id, table_number, None, None)
    ).await??;

    let mut items_resources: Vec<TableItemResource> = vec![];
    for item in items {
        let item_resource = TableItemResource::new(
            item.item_id,
            item.table_number,
            item.item_name,
            item.ordered_on,
            item.prepare_minutes,
        );
        items_resources.push(item_resource);
    }

    Ok(HttpResponse::Ok().json(items_resources))
}



#[get("/tables/{table_number}")]
pub(crate) async fn get_table(
    path: web::Path<u32>,
    data: web::Data<Pool>,
    request_id: RequestId,
) -> actix_web::Result<impl Responder> {
    let table_number = path.into_inner();

    let items = web::block(move ||
        get_table_items(&data, request_id, table_number, None, None)
    ).await??;

    let mut items_resources: Vec<TableItemResource> = vec![];
    for item in items {
        let item_resource = TableItemResource::new(
            item.item_id,
            item.table_number,
            item.item_name,
            item.ordered_on,
            item.prepare_minutes,
        );
        items_resources.push(item_resource);
    }

    let table = TableResource::new(table_number, items_resources);
    Ok(HttpResponse::Ok().json(table))
}



#[get("/tables/{table_number}/items/{item_id}")]
pub(crate) async fn get_item(
    path: web::Path<(u32, u32)>,
    data: web::Data<Pool>,
    request_id: RequestId,
) -> actix_web::Result<impl Responder> {
    let (table_number, item_id) = path.into_inner();
    let items_ids = vec![item_id];

    let response = web::block(move ||
        get_table_items(&data, request_id, table_number, items_ids.into(), None)
    ).await??;

    if let Some(item) = response.get(0) {
        let item_resource = TableItemResource::new(
            item.item_id,
            item.table_number,
            item.item_name.clone(),
            item.ordered_on.clone(),
            item.prepare_minutes,
        );
        Ok(HttpResponse::Ok().json(item_resource))
    } else {
        Err(PersistenceError::ResourceNotFound.into())
    }
}


#[delete("/tables/{table_number}/items/{item_id}")]
pub(crate) async fn remove_item(
    path: web::Path<(u32, u32)>,
    data: web::Data<Pool>,
    request_id: RequestId,
) -> actix_web::Result<impl Responder> {
    let (table_number, item_id) = path.into_inner();

    let response = web::block(move ||
        remove_table_item(&data, request_id, table_number, item_id)
    ).await??;
    Ok(HttpResponse::Ok().json(response))
}