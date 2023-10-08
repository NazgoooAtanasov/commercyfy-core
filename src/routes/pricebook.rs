use actix_web::{get, post, web, HttpResponse, Responder};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio_postgres::{Client, Row};

use crate::{
    routes::portal_user::{ErrorResponse, JWTClaims}, 
    schemas::pricebook::{CreatePricebook, CreatePricebookRecord}
};

#[derive(Debug, Serialize, Deserialize)]
struct Pricebook {
    id: uuid::Uuid,
    pricebook_name: String,
    pricebook_reference: String,
    pricebook_currency_code: String,
}

impl From<&Row> for Pricebook {
    fn from(row: &Row) -> Pricebook {
        return Pricebook {
            id: row.get("id"),
            pricebook_name: row.get("pricebook_name"),
            pricebook_reference: row.get("pricebook_reference"),
            pricebook_currency_code: row.get("pricebook_currency_code"),
        };
    }
}

#[get("/list")]
pub async fn get_pricebooks(
    app_data: web::Data<Arc<Client>>,
    request_data: Option<web::ReqData<JWTClaims>>,
) -> impl Responder {
    let claims = request_data.unwrap();

    if !claims
        .roles
        .contains(&crate::routes::portal_user::PortalUsersRoles::EDITOR)
        && !claims
            .roles
            .contains(&crate::routes::portal_user::PortalUsersRoles::ADMIN)
    {
        return HttpResponse::Unauthorized().finish();
    }

    let pricebooks_lookup_response = app_data.query("SELECT * FROM pricebooks", &[]).await;
    if let Err(error) = pricebooks_lookup_response {
        return HttpResponse::BadRequest().json(ErrorResponse {
            error_message: error.to_string(),
        });
    }

    let pricebooks: Vec<Pricebook> = pricebooks_lookup_response
        .unwrap()
        .iter()
        .map(|x| Pricebook::from(x))
        .collect();

    return HttpResponse::Ok().json(pricebooks);
}

#[post("/create")]
pub async fn create_pricebook(
    app_data: web::Data<Arc<Client>>,
    data: web::Json<CreatePricebook>,
    request_data: Option<web::ReqData<JWTClaims>>,
) -> impl Responder {
    let claims = request_data.unwrap();

    if !claims
        .roles
        .contains(&crate::routes::portal_user::PortalUsersRoles::EDITOR)
        && !claims
            .roles
            .contains(&crate::routes::portal_user::PortalUsersRoles::ADMIN)
    {
        return HttpResponse::Unauthorized().finish();
    }

    if data.pricebook_name.is_empty() {
        return HttpResponse::BadRequest().json(ErrorResponse {
            error_message: String::from("pricebook_name is required field."),
        });
    }

    if data.pricebook_reference.is_empty() {
        return HttpResponse::BadRequest().json(ErrorResponse {
            error_message: String::from("pricebook_reference is required field."),
        });
    }

    if data.pricebook_currency_code.is_empty() {
        return HttpResponse::BadRequest().json(ErrorResponse {
            error_message: String::from("pricebook_currency_code is required field."),
        });
    }

    let pricebook_create_result = app_data.query(
        "INSERT INTO pricebooks (pricebook_name, pricebook_reference, pricebook_currency_code) VALUES ($1, $2, $3)", 
        &[&data.pricebook_name, &data.pricebook_reference, &data.pricebook_currency_code]
    ).await;

    if let Err(error) = pricebook_create_result {
        return HttpResponse::BadRequest().json(ErrorResponse {
            error_message: format!(
                "There was an error creating the pricebook {}.",
                error.to_string()
            ),
        });
    }

    return HttpResponse::Created().finish();
}

#[derive(Serialize, Deserialize)]
pub struct PricebookRecord {
    pub product_id: uuid::Uuid,
    pub price: Decimal,
}

impl From<&Row> for PricebookRecord {
    fn from(value: &Row) -> Self {
        return PricebookRecord {
            product_id: value.get("product_id"),
            price: value.get("price"),
        };
    }
}

#[post("/{picebook_id}/record")]
pub async fn create_record(
    app_data: web::Data<Arc<Client>>,
    data: web::Json<CreatePricebookRecord>,
    path: web::Path<uuid::Uuid>,
    request_data: Option<web::ReqData<JWTClaims>>,
) -> impl Responder {
    let claims = request_data.unwrap();

    if !claims
        .roles
        .contains(&crate::routes::portal_user::PortalUsersRoles::EDITOR)
        && !claims
            .roles
            .contains(&crate::routes::portal_user::PortalUsersRoles::ADMIN)
    {
        return HttpResponse::Unauthorized().finish();
    }

    let existing_product_lookup = app_data
        .query_one("SELECT id FROM products WHERE id = $1", &[&data.product_id])
        .await;
    if let Err(error) = existing_product_lookup {
        return HttpResponse::BadRequest().json(ErrorResponse {
            error_message: format!(
                "No product was found or there was another error {}",
                error.to_string()
            ),
        });
    }

    let pricebook_id = path.into_inner();
    let existing_pricebook_lookup = app_data
        .query_one("SELECT id FROM pricebooks WHERE id = $1", &[&pricebook_id])
        .await;
    if let Err(error) = existing_pricebook_lookup {
        return HttpResponse::BadRequest().json(ErrorResponse {
            error_message: format!(
                "No pricebook was found or there was another error {}",
                error.to_string()
            ),
        });
    }

    let product_pricebook_record_create = app_data
        .query(
            "INSERT INTO pricebooks_products (price, product_id, pricebook_id) VALUES ($1, $2, $3)",
            &[&data.price, &data.product_id, &pricebook_id],
        )
        .await;
    if let Err(error) = product_pricebook_record_create {
        return HttpResponse::BadRequest().json(ErrorResponse {
            error_message: format!(
                "There was an error creating pricebook entry for the product: {}",
                error.to_string()
            ),
        });
    }

    return HttpResponse::Created().finish();
}
