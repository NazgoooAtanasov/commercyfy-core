use std::sync::Arc;
use postgres_types::ToSql;
use serde::{Deserialize, Serialize};
use tokio_postgres::{Client, Row, Error};
use actix_web::{web, get, Responder, HttpResponse, http::StatusCode, post};
use crate::routes::{portal_user::{ErrorResponse, JWTClaims}, inventory::ProductInventoryRecord, pricebook::PricebookRecord};

#[derive(Serialize, Deserialize, Clone, Debug, ToSql)]
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

#[derive(Serialize, Deserialize, Debug)]
pub struct Product {
    pub id: uuid::Uuid,
    pub product_name: String,
    pub product_description: String,
    pub product_color: Option<String>,
    pub product_images: Option<Vec<ProductImage>>
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

#[derive(Serialize, Deserialize, Debug)]
pub struct ProductImageInput {
    pub src: String,
    pub alt: String,
    pub srcset: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProductInput {
    product_name: String,
    product_description: String,
    product_color: Option<String>,
    product_images: Option<Vec<ProductImageInput>>,
    category_assignments: Option<Vec<uuid::Uuid>>
}

#[post("/create")]
pub async fn create_product(
    app_data: web::Data::<Arc<Client>>,
    data: web::Json<ProductInput>,
    request_data: Option<web::ReqData<JWTClaims>>
) -> impl Responder {
    let claims = request_data.unwrap();

    if !claims.roles.contains(&crate::routes::portal_user::PortalUsersRoles::EDITOR) && !claims.roles.contains(&crate::routes::portal_user::PortalUsersRoles::ADMIN) {
        return HttpResponse::Unauthorized().finish();
    }

    if data.product_name.is_empty() {
        return HttpResponse::BadRequest().json(ErrorResponse{
            error_message: "product_name is required field.".to_string()
        });
    }

    if data.product_description.is_empty() {
        return HttpResponse::BadRequest().json(ErrorResponse{
            error_message: "product_description is required field.".to_string()
        });
    }

    let product_insert_result = app_data.query_one(" \
        INSERT INTO products (product_name, product_description, product_color) \
        VALUES ($1, $2, $3) \
        RETURNING id, product_name, product_description, product_color
    ", &[&data.product_name, &data.product_description, &data.product_color]).await;

    if let Err(error) = &product_insert_result {
        return HttpResponse::BadRequest().json(ErrorResponse{
            error_message: error.to_string()
        });
    }

    let product = Product::from(&product_insert_result.unwrap());

    if let Some(images) = &data.product_images {
        if !images.is_empty() {
            let mut query: String = String::from("INSERT INTO images (product_id, src, srcset, alt) VALUES");
            let mut parameters = Vec::new();
            parameters.push(&product.id as &(dyn ToSql + Sync));

            let mut idx = 1;
            for (image_idx, image) in images.iter().enumerate() {
                if image_idx == 0 {
                    query.insert_str(query.len(), format!(" ($1, ${}, ${}, ${})", idx + 1, idx + 2, idx + 3).as_str());
                } else {
                    query.insert_str(query.len(), format!(", ($1, ${}, ${}, ${})", idx + 1, idx + 2, idx + 3).as_str());
                }
                idx += 3;

                parameters.push(&image.src as &(dyn ToSql + Sync));
                parameters.push(&image.srcset as &(dyn ToSql + Sync));
                parameters.push(&image.alt as &(dyn ToSql + Sync));
            }

            let insert_images_result = app_data.query(&query, parameters.as_slice()).await;

            if let Err(error) = insert_images_result {
                return HttpResponse::BadRequest().json(ErrorResponse {
                    error_message: error.to_string()
                });
            }
        }
    }

    if let Some(category_ids) = &data.category_assignments {
        if !category_ids.is_empty() {
            let mut query: String = String::from("INSERT INTO categories_products (product_id, category_id) values");
            let mut parameters = Vec::new();
            parameters.push(&product.id as &(dyn ToSql + Sync));

            for (idx, category_id) in category_ids.iter().enumerate() {
                if idx + 1 == 1 {
                    query.insert_str(query.len(), format!("($1, ${})", idx + 2).as_str());
                } else {
                    query.insert_str(query.len(), format!(", ($1, ${})", idx + 2).as_str());
                }

                let category_to_sql = category_id;
                parameters.push(category_to_sql as &(dyn ToSql + Sync));
            }

            let assign_categories_result = app_data.query(&query, parameters.as_slice()).await;

            if let Err(error) = assign_categories_result {
                return HttpResponse::BadRequest().json(ErrorResponse {
                    error_message: error.to_string()
                });
            }
        }
    }

    return HttpResponse::Created().finish();
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ImageInput {
    pub src: String,
    pub srcset: Option<String>,
    pub alt: Option<String>
}

#[post("/{product_id}/images/create")]
pub async fn create_images(
    app_data: web::Data::<Arc<Client>>,
    data: web::Json<Vec<ImageInput>>,
    request_data: Option<web::ReqData<JWTClaims>>,
    path: web::Path<uuid::Uuid>
) -> impl Responder {
    let claims = request_data.unwrap();

    if !claims.roles.contains(&crate::routes::portal_user::PortalUsersRoles::EDITOR) && !claims.roles.contains(&crate::routes::portal_user::PortalUsersRoles::ADMIN) {
        return HttpResponse::Unauthorized().finish();
    }

    let product_id = path.into_inner();

    let product_lookup_result = app_data.query_one("SELECT id FROM products WHERE id = $1", &[&product_id]).await;

    if let Err(error) = product_lookup_result {
        return HttpResponse::BadRequest().json(ErrorResponse {
            error_message: error.to_string()
        });
    }

    if data.is_empty() {
        return HttpResponse::BadRequest().json(ErrorResponse {
            error_message: "No images were provided.".to_string()
        });
    }

    let mut query: String = String::from("INSERT INTO images (product_id, src, srcset, alt) VALUES");
    let mut parameters = Vec::new();
    parameters.push(&product_id as &(dyn ToSql + Sync));

    let mut idx = 1;
    for (image_idx, image) in data.iter().enumerate() {
        if image_idx == 0 {
            query.insert_str(query.len(), format!(" ($1, ${}, ${}, ${})", idx + 1, idx + 2, idx + 3).as_str());
        } else {
            query.insert_str(query.len(), format!(", ($1, ${}, ${}, ${})", idx + 1, idx + 2, idx + 3).as_str());
        }
        idx += 3;

        parameters.push(&image.src as &(dyn ToSql + Sync));
        parameters.push(&image.srcset as &(dyn ToSql + Sync));
        parameters.push(&image.alt as &(dyn ToSql + Sync));
    }

    let insert_images_result = app_data.query(&query, parameters.as_slice()).await;

    if let Err(error) = insert_images_result {
        return HttpResponse::BadRequest().json(ErrorResponse {
            error_message: error.to_string()
        });
    }

    return HttpResponse::Created().finish();
}

#[get("/{product_id}/price/{pricebook_id}")]
pub async fn get_product_price(path: web::Path<(uuid::Uuid, uuid::Uuid)>, app_data: web::Data::<Arc<Client>>) -> impl Responder {
    let (product_id, pricebook_id) = path.into_inner();

    let product_price_lookup = app_data.query_one(
        "SELECT product_id, price FROM pricebooks_products WHERE product_id = $1 AND pricebook_id = $2",
        &[&product_id, &pricebook_id]).await;


    if let Err(error) = product_price_lookup {
        return HttpResponse::BadRequest().json(ErrorResponse {
            error_message: error.to_string()
        });
    }

    let record = PricebookRecord::from(&product_price_lookup.unwrap());

    return HttpResponse::Ok().json(record);
}
