use std::sync::Arc;

use actix_web::{get, web, http::StatusCode, HttpResponse, Responder, post};
use tokio_postgres::Client;

use crate::{
    routes::portal_user::JWTClaims,
    schemas::inventory::{CreateInventory, CreateInventoryRecord},
    models::{inventory::Inventory, error::ErrorResponse}
};

#[get("/{inventory_id}")]
pub async fn get_inventory(path: web::Path<String>, app_data: web::Data::<Arc<Client>>) -> impl Responder {
    let inventory_id = path.into_inner();

    let inventory_lookup_result = app_data.query("\
        SELECT p.id AS product_id, ip.allocation, i.inventory_reference, i.inventory_name, i.id as inventory_id FROM products p
        JOIN inventories_products ip ON p.id = ip.product_id
        JOIN inventories i ON ip.inventory_id = i.id
        WHERE i.id::text = $1 OR i.inventory_reference = $1;
        ", &[&inventory_id]).await;

    if let Err(_error) = inventory_lookup_result {
        return HttpResponse::build(StatusCode::BAD_REQUEST).json(ErrorResponse{ error_message: "There was and error finding the requested resource.".to_string()  });
    }

    let inventory = Inventory::from(&inventory_lookup_result.unwrap());

    return HttpResponse::build(StatusCode::OK).json(inventory);
}

#[post("/create")]
pub async fn create_inventory(app_data: web::Data::<Arc<Client>>, data: web::Json<CreateInventory>, request_data: Option<web::ReqData<JWTClaims>>) -> impl Responder {
    let claims = request_data.unwrap();

    if !claims.roles.contains(&crate::routes::portal_user::PortalUsersRoles::EDITOR) && !claims.roles.contains(&crate::routes::portal_user::PortalUsersRoles::ADMIN) {
        return HttpResponse::Unauthorized().finish();
    }

    if data.inventory_name.eq("") {
        return HttpResponse::BadRequest().json(ErrorResponse{
            error_message: "Inventory name is a required field.".to_string()
        });
    }

    if data.inventory_reference.eq("") {
        return HttpResponse::BadRequest().json(ErrorResponse{
            error_message: "Inventory reference is a required field.".to_string()
        });
    }

    let inventory_insert_result = app_data.query("\
        INSERT INTO inventories (inventory_name, inventory_reference) VALUES ($1, $2)
        ", &[&data.inventory_name, &data.inventory_reference]).await;

    if let Err(_error) = inventory_insert_result {
        return HttpResponse::BadRequest().json(ErrorResponse{ error_message: "There was an error creating inventory.".to_string()});
    }

    return HttpResponse::Created().finish();
}

#[post("/record")]
async fn create_record(
    app_data: web::Data::<Arc<Client>>,
    data: web::Json<CreateInventoryRecord>,
    request_data: Option<web::ReqData<JWTClaims>>,
) -> impl Responder {
    let claims = request_data.unwrap();

    if !claims.roles.contains(&crate::routes::portal_user::PortalUsersRoles::EDITOR) && !claims.roles.contains(&crate::routes::portal_user::PortalUsersRoles::ADMIN) {
        return HttpResponse::Unauthorized().finish();
    }

    let existing_inventory_lookip = app_data.query_one("SELECT id FROM inventories WHERE id = $1", &[&data.inventory_id]).await;
    if let Err(_error) = existing_inventory_lookip {
        return HttpResponse::BadRequest().json(ErrorResponse{ error_message: "Inventory with that id or reference does not exist.".to_string()});
    }

    let existing_product_lookip = app_data.query_one("SELECT id FROM products WHERE id = $1", &[&data.product_id]).await;
    if let Err(_error) = existing_product_lookip {
        return HttpResponse::BadRequest().json(ErrorResponse{ error_message: "Product with that id does not exist.".to_string()});
    }

    let insert_relation_query = app_data.query(
        "INSERT INTO inventories_products (product_id, inventory_id, allocation) VALUES ($1, $2, $3)",
        &[&data.product_id, &data.inventory_id, &data.allocation]).await;

    if let Err(_error) = insert_relation_query {
        println!("{:?}", _error);
        return HttpResponse::BadRequest().json(ErrorResponse{ error_message: "There was an error inserting the record.".to_string() });
    }

    return HttpResponse::Created().finish();
}
