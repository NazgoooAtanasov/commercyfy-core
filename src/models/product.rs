use serde::{Serialize, Deserialize};

#[derive(Serialize, sqlx::FromRow)]
pub struct ProductImage {
    pub id: uuid::Uuid,
    pub src: String,
    pub srcset: Option<String>,
    pub alt: Option<String>,
    pub product_id: uuid::Uuid,
}

#[derive(Serialize, Deserialize, Debug, sqlx::FromRow)]
pub struct Product {
    pub id: uuid::Uuid,
    pub product_name: String,
    pub product_description: String,
    pub product_color: Option<String>,
    // pub product_custom_fields: Option<Vec<String>>
}
