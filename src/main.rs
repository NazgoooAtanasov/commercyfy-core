mod models;
mod schemas; 
mod services;
mod routes;

use routes::category::{create_category, get_categories, get_category};
use axum::{extract::State, routing::{get, post}, serve, Router};
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

    let app = Router::new()
        .merge(categories)
        .with_state(commercyfy_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("Could not bind the TCP socket!");
    serve(listener, app)
        .await
        .expect("Could not serve the server!");
}
