use actix_web::{get, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio_postgres::{Client, Row};

use crate::routes::portal_user::ErrorResponse;

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
pub async fn get_pricebooks(app_data: web::Data<Arc<Client>>) -> impl Responder {
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
