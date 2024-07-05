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

mod routes;

#[tokio::main]
pub async fn main() {
    let account = Router::new()
        .route("/shopper-context/account", get(get_account))
        .route("/shopper-context/account", post(create_account))
        .route("/shopper-context/account/signin", post(signin));

    let app = Router::new()
        .merge(account)
        .route("/shopper-context/basket", get(get_basket))
        .route("/shopper-context/product-search", get(product_search))
        .route("/shopper-context/category-search", get(category_search));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000")
        .await
        .expect("Could not bind the TCP socket!");

    println!("Server starting");
    axum::serve(listener, app)
        .await
        .expect("Could not serve the server");
}
