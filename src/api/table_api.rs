use actix_request_identifier::RequestId;
use crate::model::request_model::{
    AddItemsToTableRequest
};
use crate::repository::remove_table_items::remove_table_item;
use actix_web::{delete, get, put, web, HttpResponse, Responder};
use mysql::Pool;
use crate::repository::persist_table_items::add_items_to_table;
use crate::repository::fetch_table_items::get_table_items;


#[put("/tables/{table_number}")]
pub(crate) async fn add_items(
    path: web::Path<u32>,
    web::Json(request): web::Json<AddItemsToTableRequest>,
    data: web::Data<Pool>,
    request_id: RequestId
) -> actix_web::Result<impl Responder> {
    let table_number = path.into_inner();

    let response = web::block(move ||
        add_items_to_table(
            &data, request_id,
            table_number,
            request.items_names,
        )).await??;
    Ok(HttpResponse::Ok().json(response))
}


#[get("/tables/{table_number}")]
pub(crate) async fn get_table(
    path: web::Path<u32>,
    data: web::Data<Pool>,
    request_id: RequestId
) -> actix_web::Result<impl Responder> {
    let table_number = path.into_inner();

    let response = web::block(move ||
        get_table_items(&data, request_id, table_number, None, None)
    ).await??;
    Ok(HttpResponse::Ok().json(response))
}


#[get("/tables/{table_number}/items/{item_id}")]
pub(crate) async fn get_item(
    path: web::Path<(u32, u32)>,
    data: web::Data<Pool>,
    request_id: RequestId
) -> actix_web::Result<impl Responder> {
    let (table_number, item_id) = path.into_inner();
    let items_ids = vec![item_id];

    let response = web::block(move ||
        get_table_items(&data, request_id, table_number, items_ids.into(), None)
    ).await??;
    Ok(HttpResponse::Ok().json(response))
}


#[delete("/tables/{table_number}/items/{item_id}")]
pub(crate) async fn remove_item(
    path: web::Path<(u32, u32)>,
    data: web::Data<Pool>,
    request_id: RequestId
) -> actix_web::Result<impl Responder> {
    let (table_number, item_id) = path.into_inner();

    let response = web::block(move ||
        remove_table_item(&data, request_id, table_number, item_id)
    ).await??;
    Ok(HttpResponse::Ok().json(response))
}