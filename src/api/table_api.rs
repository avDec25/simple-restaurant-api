use actix_request_identifier::RequestId;
use crate::model::request_model::{
    AddItemsToTableRequest, ListTableItemsRequest, RemoveTableItemRequest,
};
use crate::repository::remove_table_items::remove_table_item;
use actix_web::{delete, get, post, web, HttpResponse, Responder};
use mysql::Pool;
use crate::repository::persist_table_items::add_items_to_table;
use crate::repository::fetch_table_items::get_table_items;

#[post("/table/add_items")]
pub(crate) async fn add_items(
    web::Json(request): web::Json<AddItemsToTableRequest>,
    data: web::Data<Pool>,
    request_id: RequestId
) -> actix_web::Result<impl Responder> {
    let response = web::block(move ||
        add_items_to_table(
            &data, request_id,
            request.table_number,
            request.items_names,
        )).await??;
    Ok(HttpResponse::Ok().json(response))
}

#[get("/table/list_items")]
pub(crate) async fn get_items(
    web::Json(request): web::Json<ListTableItemsRequest>,
    data: web::Data<Pool>,
    request_id: RequestId
) -> actix_web::Result<impl Responder> {
    let response = web::block(move ||
        get_table_items(&data, request_id, request.table_number, request.items_ids, request.items_names)
    ).await??;
    Ok(HttpResponse::Ok().json(response))
}

#[delete("/table/remove_item")]
pub(crate) async fn remove_item(
    web::Json(request): web::Json<RemoveTableItemRequest>,
    data: web::Data<Pool>,
    request_id: RequestId
) -> actix_web::Result<impl Responder> {
    let response = web::block(move ||
        remove_table_item(&data, request_id, request.item_id)
    ).await??;
    Ok(HttpResponse::Ok().json(response))
}