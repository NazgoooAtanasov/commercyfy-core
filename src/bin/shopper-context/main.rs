use std::sync::Arc;

use axum::{
    routing::{get, post},
    Router,
};
use commercyfy_core::services::db::PgDbService;
use routes::{
    account::{create_account, get_account, signin},
    basket::get_basket,
    category_search::category_search,
    product_search::product_search,
    ShopperContextState,
};
use sqlx::postgres::PgPoolOptions;
use tower_sessions::{cookie::time::Duration, Expiry, MemoryStore, SessionManagerLayer};

mod extractors;
mod routes;

#[tokio::main]
pub async fn main() {
    dotenv::dotenv().expect("Could not load env variables!");
    let db_connstr = std::env::var("DATABASE_URL").expect("DATABASE_URL was not found in .env!");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_connstr)
        .await
        .expect("Could not connect to the database!");

    let request_state = Arc::new(ShopperContextState {
        db_service: PgDbService::new(pool),
    });

    let session_store = MemoryStore::default();
    let session_layer = SessionManagerLayer::new(session_store)
        .with_name("shopper-context-sid")
        .with_expiry(Expiry::OnInactivity(Duration::seconds(10)));

    let account = Router::new()
        .route("/shopper-context/account", get(get_account))
        .route("/shopper-context/account", post(create_account))
        .route("/shopper-context/account/signin", post(signin));

    let basket = Router::new().route("/shopper-context/basket", get(get_basket));

    let app = Router::new()
        .merge(account)
        .merge(basket)
        .route("/shopper-context/product-search", get(product_search))
        .route("/shopper-context/category-search", get(category_search))
        .layer(session_layer)
        .with_state(request_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000")
        .await
        .expect("Could not bind the TCP socket!");

    println!("Server starting");
    axum::serve(listener, app)
        .await
        .expect("Could not serve the server");
}
