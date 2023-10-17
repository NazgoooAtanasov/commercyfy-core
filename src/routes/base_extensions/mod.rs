use crate::migration_generator::MigrationGenerator;
use crate::{
    models::{base_extensions::MigrationGenerated, error::ErrorResponse},
    schemas::base_extensions::{CreateExtension, CreateMigrationUpdate},
};
use actix_web::{post, web, HttpResponse, Responder};
use std::{io::Read, sync::Arc};
use tokio_postgres::Client;

#[post("/process")]
pub async fn process(data: web::Json<CreateExtension>) -> impl Responder {
    let mut generator = MigrationGenerator::new();
    generator.set_what_to_extend(data.r#type);
    for field in &data.fields {
        generator.set_field(field.clone());
    }

    let migration = generator.generate();

    match migration {
        Ok(migration_file_path) => {
            return HttpResponse::Ok().json(MigrationGenerated {
                file_path: migration_file_path,
            });
        }

        Err(error) => {
            return HttpResponse::BadRequest().json(ErrorResponse {
                error_message: error,
            });
        }
    }
}

#[post("/migrate")]
pub async fn migrate(
    data: web::Json<CreateMigrationUpdate>,
    app_data: web::Data<Arc<Client>>,
) -> impl Responder {
    let file_path = data.file_path.clone();

    let migration_file = std::fs::File::open(file_path);
    if let Err(e) = migration_file {
        return HttpResponse::BadRequest().json(ErrorResponse {
            error_message: e.to_string(),
        });
    }

    let mut migration = String::new();
    if let Err(e) = migration_file.unwrap().read_to_string(&mut migration) {
        return HttpResponse::BadRequest().json(ErrorResponse {
            error_message: e.to_string(),
        });
    }

    // @FIXME: this here is quite unfortunate. Would need to figure out how to create a
    // transaction.
    let migration_lines: Vec<&str> = migration.split("\n").collect();
    for line in migration_lines {
        if let Err(e) = app_data.query(line, &[]).await {
            return HttpResponse::BadRequest().json(ErrorResponse {
                error_message: e.to_string()
            });
        }
    }

    return HttpResponse::Ok().finish();
}
