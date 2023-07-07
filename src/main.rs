use std::sync::Arc;

use env_logger::Env;
use routes::inventory::{get_inventory, create_inventory, create_record};
use routes::portal_user::signin;
use routes::product::get_product_inventory;
use tokio_postgres::{Config, NoTls, Error};
use actix_web::{HttpServer, App, middleware::Logger, web};

mod routes;
mod middlewares;
use crate::routes::product::get_product;
use crate::routes::category::get_categories;
use crate::routes::portal_user::create_user;
use crate::middlewares::authentication::Authentication;

#[actix_web::main]
async fn main() -> Result<(), Error> {
    dotenv::dotenv().ok();

    let (client, conn) = Config::new()
        .host(std::env::var("POSTGRES_HOST").expect("POSTGRES_HOST should be set in .env.").as_str())
        .user(std::env::var("POSTGRES_USER").expect("POSTGRES_USER should be set in .env.").as_str())
        .password(std::env::var("POSTGRES_PASSWORD").expect("POSTGRES_PASSWORD should be set in .env.").as_str())
        .dbname(std::env::var("POSTGRES_DB").expect("POSTGRES_DB should be set in .env.").as_str())
        .connect(NoTls).await?;
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

            .service(
                web::scope("/product")
                    .wrap(Authentication)
                    .service(get_product)
                    .service(get_product_inventory)
            )

            .service(
                web::scope("/categories")
                    .wrap(Authentication)
                    .service(get_categories)
            )

            .service(
                web::scope("/portal")
                    .wrap(Authentication)
                    .service(create_user)
            )

            .service(
                web::scope("/inventory")
                    .wrap(Authentication)
                    .service(get_inventory)
                    .service(create_inventory)
                    .service(create_record)
            )

            .service(
                web::scope("/portal-user")
                    .service(signin)
            )

            .wrap(Logger::default())
    })
        .bind(("localhost", 8080)).unwrap_or_else(|_| {
            eprintln!("[ERROR] Failed binding to socket");
            std::process::exit(1);
        })
        .run()
        .await
        .unwrap();

    return Ok(());
}
