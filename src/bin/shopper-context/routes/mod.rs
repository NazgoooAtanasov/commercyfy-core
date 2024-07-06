pub mod account;
pub mod basket;
pub mod category_search;
pub mod product_search;

use commercyfy_core::services::db::PgDbService;

pub struct ShopperContextState {
    pub db_service: PgDbService,
}
