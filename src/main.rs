use std::sync::Arc;

use actix_web::{error, HttpResponse};
use actix_web::{middleware::Logger, web, App, HttpServer};
use env_logger::Env;
use models::error::ErrorResponse;
use routes::base_extensions::{process, migrate};
use routes::category::{assing_products, create_category, get_category};
use routes::inventory::{create_inventory, get_inventory};
use routes::portal_user::signin;
use routes::pricebook::{create_pricebook, get_pricebooks};
use routes::product::{create_images, create_product, get_product_inventory, get_product_price};
use tokio_postgres::{Error, NoTls};

mod middlewares;
mod models;
mod routes;
mod schemas;
mod migration_generator;
use crate::middlewares::authentication::Authentication;
use crate::routes::category::get_categories;
use crate::routes::portal_user::create_user;
use crate::routes::product::get_product;

#[actix_web::main]
async fn main() -> Result<(), Error> {
    dotenv::dotenv().ok();

    let (client, conn) = tokio_postgres::connect(
        std::env::var("DATABASE_URL").unwrap().as_str(),
        NoTls
    ).await?;

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
                    .service(get_product_price),
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
            .service(
                web::scope("/extensions")
                    // .wrap(Authentication)
                    .service(process)
                    .service(migrate),
            )
            .wrap(Logger::default())
    })
    .bind(("0.0.0.0", std::env::var("PORT").unwrap().parse().unwrap()))
    .unwrap_or_else(|_| {
        eprintln!("[ERROR] Failed binding to socket");
        std::process::exit(1);
    })
    .run()
    .await
    .unwrap();

    return Ok(());
}
