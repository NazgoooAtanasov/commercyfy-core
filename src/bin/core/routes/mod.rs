use axum::extract::State;
use commercyfy_core::services::{
    db::PgDbService, logger::GenericLogger, role_validation::RoleValidation,
    unstructureddb::MongoDb,
};
use std::sync::Arc;

#[derive(serde::Serialize, Debug)]
pub struct CreatedEntryResponse {
    pub id: sqlx::types::Uuid,
}

pub struct CommercyfyState {
    pub db_service: PgDbService,
    pub role_service: RoleValidation,
    pub unstructureddb: MongoDb,
    pub logger: GenericLogger,
}

type CommercyfyExtrState = State<Arc<CommercyfyState>>;

pub mod base_extensions;
pub mod category;
pub mod inventory;
pub mod logs;
pub mod portal;
pub mod pricebook;
pub mod product;
