use std::sync::Arc;

use actix_web::{error, HttpResponse};
use actix_web::{middleware::Logger, web, App, HttpServer};
use env_logger::Env;
use models::error::ErrorResponse;
use routes::category::{assing_products, create_category, get_category};
use routes::inventory::{create_inventory, get_inventory};
use routes::portal_user::signin;
use routes::pricebook::{create_pricebook, get_pricebooks};
use routes::product::{create_images, create_product, get_product_inventory, get_product_price};
use tokio_postgres::{Config, Error, NoTls};

mod schemas;
mod models;
mod middlewares;
mod routes;
use crate::middlewares::authentication::Authentication;
use crate::routes::category::get_categories;
use crate::routes::portal_user::create_user;
use crate::routes::product::get_product;

#[actix_web::main]
async fn main() -> Result<(), Error> {
    dotenv::dotenv().ok();

    let (client, conn) = Config::new()
        .host(
            std::env::var("POSTGRES_HOST")
                .expect("POSTGRES_HOST should be set in .env.")
                .as_str(),
        )
        .user(
            std::env::var("POSTGRES_USER")
                .expect("POSTGRES_USER should be set in .env.")
                .as_str(),
        )
        .password(
            std::env::var("POSTGRES_PASSWORD")
                .expect("POSTGRES_PASSWORD should be set in .env.")
                .as_str(),
        )
        .dbname(
            std::env::var("POSTGRES_DB")
                .expect("POSTGRES_DB should be set in .env.")
                .as_str(),
        )
        .connect(NoTls)
        .await?;
    let client = Arc::new(client);

    tokio::spawn(async move {
        if let Err(err) = conn.await {
            eprintln!("Connection error {}", err);
        } else {
            println!("Connected successfully");
        }
    });

    env_logger::init_from_env(Env::default().default_filter_or("info"));
    HttpServer::new(move || {
        let client = Arc::clone(&client);
        App::new()
            .app_data(web::Data::new(client))
            .app_data(web::JsonConfig::default().error_handler(|err, _req| {
                return error::InternalError::from_response(
                    "",
                    HttpResponse::BadRequest()
                        .json(ErrorResponse {
                            error_message: err.to_string(),
                        })
                        .into(),
                )
                .into();
            }))
            .service(
                web::scope("/product")
                    .wrap(Authentication)
                    .service(get_product)
                    .service(get_product_inventory)
                    .service(create_product)
                    .service(create_images)
                    .service(get_product_price)
            )
            .service(
                web::scope("/categories")
                    .wrap(Authentication)
                    .service(get_categories)
                    .service(create_category)
                    .service(assing_products)
                    .service(get_category),
            )
            .service(
                web::scope("/portal")
                    .wrap(Authentication)
                    .service(create_user),
            )
            .service(
                web::scope("/inventory")
                    .wrap(Authentication)
                    .service(get_inventory)
                    .service(create_inventory)
                    .service(routes::inventory::create_record),
            )
            .service(web::scope("/portal-user").service(signin))
            .service(
                web::scope("/pricebooks")
                    .wrap(Authentication)
                    .service(get_pricebooks)
                    .service(create_pricebook)
                    .service(routes::pricebook::create_record),
            )
            .wrap(Logger::default())
    })
    .bind(("localhost", 8080))
    .unwrap_or_else(|_| {
        eprintln!("[ERROR] Failed binding to socket");
        std::process::exit(1);
    })
    .run()
    .await
    .unwrap();

    return Ok(());
}
