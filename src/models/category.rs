use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Category {
    pub id: sqlx::types::Uuid,
    pub category_name: String,
    pub category_description: Option<String>,
    pub category_reference: String,
}
