use actix_web::{post, web, HttpResponse, Responder};
use mysql::Pool;
use crate::model::table_model::AddItemsToTableRequest;
use crate::repository::persistence::add_items_to_table;

#[post("/table/add_items")]
pub(crate) async fn add_items(
    web::Json(request): web::Json<AddItemsToTableRequest>,
    data: web::Data<Pool>,
) -> actix_web::Result<impl Responder> {
    web::block(move || add_items_to_table(&data, request.table_num, request.items_name)).await?;
    Ok(HttpResponse::NoContent())
}