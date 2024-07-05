use axum::{
    routing::{get, post},
    Router,
};
use routes::{
    account::{create_account, get_account, signin},
    basket::get_basket,
    category_search::category_search,
    product_search::product_search,
};
use tower_sessions::{Expiry, MemoryStore, SessionManagerLayer, cookie::time::Duration};

mod routes;
mod extractors;

#[tokio::main]
pub async fn main() {
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
        .layer(session_layer);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000")
        .await
        .expect("Could not bind the TCP socket!");

    println!("Server starting");
    axum::serve(listener, app)
        .await
        .expect("Could not serve the server");
}
