mod models;
mod routes;
mod schemas;
mod services;

use axum::{
    extract::State,
    routing::{get, post},
    serve, Router,
};
use routes::{
    category::{create_category, get_categories, get_category},
    product::{create_product, create_product_image, get_product},
};
use services::db::PgDbService;
use sqlx::postgres::PgPoolOptions;
use std::sync::Arc;

pub struct CommercyfyState {
    pub db_service: PgDbService,
}

type CommercyfyExtrState = State<Arc<CommercyfyState>>;

#[tokio::main]
pub async fn main() {
    dotenv::dotenv().expect("Could not load env variables!");

    let db_connstr = std::env::var("DATABASE_URL").expect("DATABASE_URL was not found in .env!");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_connstr)
        .await
        .expect("Could not connect to the database!");

    let db_service = PgDbService::new(pool);
    let commercyfy_state = Arc::new(CommercyfyState { db_service });

    let categories = Router::new()
        .route("/categories", get(get_categories))
        .route("/categories", post(create_category))
        .route("/categories/:id", get(get_category));

    let product = Router::new()
        .route("/product/:id", get(get_product))
        .route("/product", post(create_product))
        .route("/product/:id/images", post(create_product_image));

    let app = Router::new()
        .merge(categories)
        .merge(product)
        .with_state(commercyfy_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("Could not bind the TCP socket!");
    serve(listener, app)
        .await
        .expect("Could not serve the server!");
}
