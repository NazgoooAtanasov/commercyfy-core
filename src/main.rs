mod middlewares;
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
    inventory::{
        create_inventory, create_inventory_record, get_inventories, get_inventory,
        get_inventory_record,
    },
    portal::{create_portal_user, get_portal_user, signin_portal_user},
    pricebook::{
        create_pricebook, create_pricebook_record, get_pricebook, get_pricebook_record,
        get_pricebooks,
    },
    product::{create_product, create_product_image, get_product},
};
use services::{db::PgDbService, role_validation::RoleValidation};
use sqlx::postgres::PgPoolOptions;
use std::sync::Arc;

pub struct CommercyfyState {
    pub db_service: PgDbService,
    pub role_service: RoleValidation 
}

type CommercyfyExtrState = State<Arc<CommercyfyState>>;

#[tokio::main]
pub async fn main() {
    dotenv::dotenv().expect("Could not load env variables!");
    std::env::var("JWT_TOKEN_SECRET").expect("JWT_TOKEN_SECRET was not found in .env!");

    let db_connstr = std::env::var("DATABASE_URL").expect("DATABASE_URL was not found in .env!");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_connstr)
        .await
        .expect("Could not connect to the database!");

    let db_service = PgDbService::new(pool);
    let role_service = RoleValidation::default();
    let commercyfy_state = Arc::new(CommercyfyState {
        db_service,
        role_service,
    });

    let categories = Router::new()
        .route("/categories", get(get_categories))
        .route("/categories", post(create_category))
        .route("/categories/:id", get(get_category));

    let product = Router::new()
        .route("/product/:id", get(get_product))
        .route("/product", post(create_product))
        .route("/product/:id/images", post(create_product_image));

    let inventory = Router::new()
        .route("/inventories", get(get_inventories))
        .route("/inventory/:id", get(get_inventory))
        .route("/inventory", post(create_inventory))
        .route("/inventory/record", post(create_inventory_record))
        .route(
            "/inventory/:inventory/record/:product",
            get(get_inventory_record),
        );

    let pricebooks = Router::new()
        .route("/pricebooks", get(get_pricebooks))
        .route("/pricebook/:id", get(get_pricebook))
        .route("/pricebook", post(create_pricebook))
        .route("/pricebook/record", post(create_pricebook_record))
        .route(
            "/pricebook/:pricebook/record/:product",
            get(get_pricebook_record),
        );

    let portal = Router::new()
        .route("/portal/user/:id", get(get_portal_user))
        .route("/portal/user", post(create_portal_user));

    let auth_routes = Router::new()
        .merge(categories)
        .merge(product)
        .merge(inventory)
        .merge(pricebooks)
        .merge(portal)
        .route_layer(axum::middleware::from_fn(middlewares::authentication::auth));

    let signin = Router::new().route("/portal/signin", post(signin_portal_user));

    let app = Router::new()
        .merge(auth_routes)
        .merge(signin)
        .with_state(commercyfy_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("Could not bind the TCP socket!");
    serve(listener, app)
        .await
        .expect("Could not serve the server!");
}
