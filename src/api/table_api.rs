use crate::model::table_model::{AddItemsToTableRequest, ListTableItemsRequest, RemoveTableItemRequest};
use crate::repository::persistence::get_table_items;
use crate::repository::persistence::{add_items_to_table, remove_table_item};
use actix_web::{delete, get, post, web, HttpResponse, Responder};
use mysql::Pool;

#[post("/table/add_items")]
pub(crate) async fn add_items(
    web::Json(request): web::Json<AddItemsToTableRequest>,
    data: web::Data<Pool>,
) -> actix_web::Result<impl Responder> {
    web::block(move || add_items_to_table(&data, request.table_number, request.items_names)).await?;
    Ok(HttpResponse::NoContent())
}

#[get("/table/list_items")]
pub(crate) async fn get_items(
    web::Json(request): web::Json<ListTableItemsRequest>,
    data: web::Data<Pool>,
) -> actix_web::Result<impl Responder> {
    web::block(move || get_table_items(&data,
                                       request.table_number,
                                       request.items_ids,
                                       request.items_names
    )).await?;
    Ok(HttpResponse::NoContent())
}

#[delete("/table/remove_item")]
pub(crate) async fn remove_item(
    web::Json(request): web::Json<RemoveTableItemRequest>,
    data: web::Data<Pool>,
) -> actix_web::Result<impl Responder> {
    web::block(move || remove_table_item(&data, request.item_id)).await?;
    Ok(HttpResponse::NoContent())
}