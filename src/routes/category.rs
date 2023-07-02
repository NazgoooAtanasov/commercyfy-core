use std::sync::Arc;
use actix_web::{web, get, Responder, HttpResponse, http::StatusCode};
use tokio_postgres::{Client, Row};
use serde::{Deserialize, Serialize};

use crate::routes::product::Product;

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
                JOIN (SELECT DISTINCT ON (product_id) * FROM images) AS i ON i.product_id = p.id \
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
