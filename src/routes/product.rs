use std::sync::Arc;
use serde::{Deserialize, Serialize};
use tokio_postgres::{Client, Row, Error};
use actix_web::{web, get, Responder, HttpResponse, http::StatusCode};

use crate::routes::{portal_user::ErrorResponse, inventory::{Inventory, ProductInventoryRecord}};


#[derive(Serialize, Deserialize, Clone)]
pub struct ProductImage {
    id: uuid::Uuid,
    src: String,
    srcset: Option<String>,
    alt: Option<String>,
    product_id: uuid::Uuid,
}

impl From<&Row> for ProductImage {
    fn from(value: &Row) -> Self {
        return Self {
            id: value.get("image_id"),
            src: value.get("src"),
            srcset: value.try_get("srcset").map_or(None, |x| Some(x)),
            alt: value.try_get("alt").map_or(None, |x| Some(x)),
            product_id: value.get("product_id")
        };
    }
}

#[derive(Serialize, Deserialize)]
pub struct Product {
    id: uuid::Uuid,
    product_name: String,
    product_description: String,
    product_color: Option<String>,
    product_images: Option<Vec<ProductImage>>
}

impl From<&Row> for Product {
    fn from(value: &Row) -> Self {
        let mut product = Self{
            id: value.get("id"),
            product_name: value.get("product_name"),
            product_description: value.get("product_description"),
            product_color: value
                .try_get("product_color")
                .map_or(None, |x| Some(x)),
            product_images: None
        };

        // this check is needed because everywhere, besides the /product/{product_id} route, we
        // fetch only the first image of the product - thus it being part of the product query response
        let should_parse_image: Result<uuid::Uuid, Error> = value.try_get("image_id");
        if let Ok(_) = should_parse_image {
            let image = ProductImage::try_from(value);
            if let Ok(image) = image {
                product.product_images = Some(vec![image]);
            }
        }

        return product;
    }
}

#[get("/{product_id}")]
pub async fn get_product(path: web::Path<String>, app_data: web::Data::<Arc<Client>>) -> impl Responder {
    let product_id = path.into_inner();

    let product_lookup_result = app_data.query_one("SELECT * FROM products WHERE id::text = $1", &[&product_id]).await;

    if let Ok(result) = product_lookup_result {
        let mut product = Product::from(&result);

        let image_lookup_result = app_data.query("SELECT id as image_id, src, srcset, alt, product_id FROM images WHERE product_id::text = $1", &[&product_id]).await;

        if let Ok(result) = image_lookup_result {
            let images: Vec<ProductImage> = result.iter().map(|x| ProductImage::from(x)).collect();
            product.product_images = Some(images);
        }

        return HttpResponse::build(StatusCode::OK)
            .json(product);
    }

    if let Err(err) = product_lookup_result {
        return HttpResponse::build(StatusCode::BAD_REQUEST)
            .append_header(("content-type", "application/json"))
            .body(format!("{{ \"message\": \"{}\", \"product_id\": \"{}\" }}", err.to_string(), product_id));
    }

    return HttpResponse::build(StatusCode::BAD_REQUEST).finish();
}

#[get("/{product_id}/inventory/{inventory_id}")]
pub async fn get_product_inventory(path: web::Path<(String, String)>, app_data: web::Data::<Arc<Client>>) -> impl Responder {
    let (product_id, inventory_id) = path.into_inner();

    let product_inventory_lookup_result = app_data.query_one("\
        SELECT ip.allocation, ip.product_id FROM inventories_products ip WHERE ip.product_id::text = $1 AND ip.inventory_id::text = $2;
        ", &[&product_id, &inventory_id]).await;

    if let Err(_error) = product_inventory_lookup_result {
        return HttpResponse::build(StatusCode::BAD_REQUEST)
            .json(ErrorResponse{ error_message: "There was an error trying to retrieve the requested resource.".to_string()});
    }

    let product_inventory_record = ProductInventoryRecord::from(&product_inventory_lookup_result.unwrap());

    return HttpResponse::Ok().json(product_inventory_record);
}
