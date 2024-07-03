use axum::{routing::get, Router};
use routes::basket::get_basket;

mod routes;

#[tokio::main]
pub async fn main() {
    let app = Router::new()
        .route("/shopper-context/basket", get(get_basket));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000")
        .await
        .expect("Could not bind the TCP socket!");

    println!("Server starting");
    axum::serve(listener, app)
        .await
        .expect("Could not serve the server");
}
