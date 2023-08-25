use std::sync::Arc;
use actix_web::{web, get, Responder, HttpResponse, http::StatusCode, post};
use postgres_types::ToSql;
use tokio_postgres::{Client, Row};
use serde::{Deserialize, Serialize};

use crate::routes::{product::Product, portal_user::ErrorResponse};

use super::portal_user::{JWTClaims, PortalUsersRoles};

#[derive(Serialize, Deserialize)]
pub struct Category {
    id: uuid::Uuid,
    category_name: String,
    category_description: Option<String>,
    category_reference: String,
    products: Option<Vec<Product>>
}

impl From<&Row> for Category {
    fn from(value: &Row) -> Self {
        return Self {
            id: value.get("id"),
            category_name: value.get("category_name"),
            category_description: value.try_get("category_description").map_or(None, |x| Some(x)),
            category_reference: value.get("category_reference"),
            products: None
        };
    }
}

#[get("/list")]
pub async fn get_categories(app_data: web::Data<Arc<Client>>) -> impl Responder {
    let category_lookup_response = app_data.query("SELECT * FROM categories", &[]).await;

    if let Ok(result) = category_lookup_response {
        let mut categories: Vec<Category> = result.iter().map(|x| { return Category::from(x); }).collect();

        for category in &mut categories {
            let category_products_lookup = app_data.query("\
                SELECT p.id, p.product_name, p.product_description, p.product_color, i.id as image_id, i.src, i.srcset, i.alt, i.product_id FROM products p \
                JOIN categories_products cp ON p.id = cp.product_id \
                FULL OUTER JOIN (SELECT DISTINCT ON (product_id) * FROM images) AS i ON i.product_id = p.id \
                WHERE cp.category_id = $1 \
                ", &[&category.id]).await;


            if let Ok(products) = category_products_lookup {
                let products: Vec<Product> = products.iter().map(|x| { return Product::from(x); }).collect();
                category.products = Some(products);
            }
        }


        return HttpResponse::build(StatusCode::OK)
            .json(categories);
    }

    return HttpResponse::build(StatusCode::SERVICE_UNAVAILABLE).finish();
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CategoryInput {
    category_name: String,
    category_description: Option<String>,
    category_reference: String
}

#[post("/create")]
pub async fn create_category(
    app_data: web::Data<Arc<Client>>, 
    request_data: Option<web::ReqData<JWTClaims>>,
    data: web::Json<CategoryInput>,
) -> impl Responder {
    let claims = request_data.unwrap();

    if !claims.roles.contains(&PortalUsersRoles::EDITOR) && !claims.roles.contains(&PortalUsersRoles::ADMIN) {
        return HttpResponse::Unauthorized().finish();
    }

    if data.category_name.is_empty() {
        return HttpResponse::BadRequest().json(ErrorResponse {
            error_message: "category_name is a required field.".to_string()
        });
    }

    if data.category_reference.is_empty() {
        return HttpResponse::BadRequest().json(ErrorResponse {
            error_message: "category_reference name is a required field.".to_string()
        });
    }

    let category_insertion_result = app_data.query("INSERT INTO categories (category_name, category_description, category_reference) VALUES ($1, $2, $3)", 
                                                       &[&data.category_name, &data.category_description, &data.category_reference]).await;

    if let Err(error) = category_insertion_result {
        return HttpResponse::BadRequest().json(ErrorResponse {
            error_message: error.to_string()
        });
    }

    return HttpResponse::Created().finish();
}

#[post("/{category_id}/product/assign")]
pub async fn assing_products(
    app_data: web::Data::<Arc<Client>>,
    data: web::Json<Vec<uuid::Uuid>>,
    request_data: Option<web::ReqData<JWTClaims>>,
    path: web::Path<uuid::Uuid>
) -> impl Responder {
    let claims = request_data.unwrap();

    if !claims.roles.contains(&PortalUsersRoles::EDITOR) && !claims.roles.contains(&PortalUsersRoles::ADMIN) {
        return HttpResponse::Unauthorized().finish();
    }

    if data.is_empty() {
        return HttpResponse::BadRequest().json(ErrorResponse {
            error_message: "No product ids where provided.".to_string()
        });
    }

    let category_id = path.into_inner();
    let category_lookup_response = app_data.query_one("SELECT id from categories where id = $1", &[&category_id]).await;
    if let Err(error) = category_lookup_response {
        return HttpResponse::BadRequest().json(ErrorResponse {
            error_message: error.to_string()
        });
    }

    let mut query: String = String::from("INSERT INTO categories_products (category_id, product_id) values");
    let mut parameters = Vec::new();
    parameters.push(&category_id as &(dyn ToSql + Sync));

    for (idx, product_id) in data.iter().enumerate() {
        if idx + 1 == 1 {
            query.insert_str(query.len(), format!("($1, ${})", idx + 2).as_str());
        } else {
            query.insert_str(query.len(), format!(", ($1, ${})", idx + 2).as_str());
        }

        let product_to_sql = product_id;
        parameters.push(product_to_sql as &(dyn ToSql + Sync));
    }

    let assign_products_result = app_data.query(&query, parameters.as_slice()).await;

    if let Err(error) = assign_products_result {
        return HttpResponse::BadRequest().json(ErrorResponse {
            error_message: error.to_string()
        });
    }
    return HttpResponse::Ok().finish();
}
