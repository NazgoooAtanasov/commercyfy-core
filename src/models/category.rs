use serde::{Serialize, Deserialize};
use tokio_postgres::Row;
use super::product::Product;

#[derive(Serialize, Deserialize)]
pub struct Category {
    pub id: uuid::Uuid,
    pub category_name: String,
    pub category_description: Option<String>,
    pub category_reference: String,
    pub products: Option<Vec<Product>>
}

impl From<&Row> for Category {
    fn from(value: &Row) -> Self {
        return Self {
            id: value.get("id"),
            category_name: value.get("category_name"),
            category_description: value.try_get("category_description").map_or(None, |x| Some(x)),
            category_reference: value.get("category_reference"),
            products: None
        };
    }
}
