mod middlewares;
mod models;
mod routes;
mod schemas;
mod services;
mod utils;

use axum::{
    extract::State,
    routing::{get, post},
    serve, Router,
};
use routes::{
    base_extensions::{create_extension, get_extensions},
    category::{assign_products_to_category, create_category, get_categories, get_category},
    inventory::{
        create_inventory, create_inventory_record, get_inventories, get_inventory,
        get_inventory_record,
    },
    logs::{create_log, get_logs},
    portal::{create_portal_user, get_portal_user, signin_portal_user},
    pricebook::{
        create_pricebook, create_pricebook_record, get_pricebook, get_pricebook_record,
        get_pricebooks,
    },
    product::{create_product, create_product_image, get_product, get_products},
};
use services::{
    db::PgDbService,
    logger::GenericLogger,
    role_validation::RoleValidation,
    unstructureddb::MongoDb,
};
use sqlx::postgres::PgPoolOptions;
use std::sync::Arc;

pub struct CommercyfyState {
    pub db_service: PgDbService,
    pub role_service: RoleValidation,
    pub unstructureddb: MongoDb,
    pub logger: GenericLogger,
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

    let mongo_client = mongodb::Client::with_uri_str(
        std::env::var("MONGODB_URL").expect("MONGODB_URL was not found in .env!"),
    )
    .await
    .expect("Could not connect to mongodb!");
    let mongodb = mongo_client.database("commercyfy-core");

    let db_service = PgDbService::new(pool);
    let role_service = RoleValidation::default();
    let unstructureddb = MongoDb::new(mongodb);
    let logger = GenericLogger::new();

    unstructureddb
        .validate_collections()
        .await
        .expect("There was an error with creating the needed collections for the usntructureddb.");

    let commercyfy_state = Arc::new(CommercyfyState {
        db_service,
        role_service,
        unstructureddb,
        logger,
    });

    let categories = Router::new()
        .route("/categories", get(get_categories))
        .route("/categories", post(create_category))
        .route("/categories/:id", get(get_category))
        .route(
            "/categories/assign/products",
            post(assign_products_to_category),
        );

    let product = Router::new()
        .route("/products", get(get_products))
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

    let logs = Router::new()
        .route("/logs", get(get_logs))
        .route("/logs", post(create_log));

    let auth_routes = Router::new()
        .merge(categories)
        .merge(product)
        .merge(inventory)
        .merge(pricebooks)
        .merge(portal)
        .merge(logs)
        .route_layer(axum::middleware::from_fn(middlewares::authentication::auth));

    let signin = Router::new().route("/portal/signin", post(signin_portal_user));

    let extensions = Router::new()
        .route("/extensions/:object", get(get_extensions))
        .route("/extensions", post(create_extension));

    let app = Router::new()
        .merge(auth_routes)
        .merge(signin)
        .merge(extensions)
        .with_state(commercyfy_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("Could not bind the TCP socket!");
    serve(listener, app)
        .await
        .expect("Could not serve the server!");
}
