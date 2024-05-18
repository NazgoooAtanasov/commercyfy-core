use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateProductImage {
    pub src: String,
    pub srcset: Option<String>,
    pub alt: Option<String>
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum CustomField {
    STRING(String),
    BOOLEAN(bool)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateProduct {
    pub product_name: String,
    pub product_description: String,
    pub product_color: Option<String>,
    pub product_images: Option<Vec<CreateProductImage>>,
    pub category_assignments: Option<Vec<uuid::Uuid>>,
    pub custom_fields: Option<HashMap<String, CustomField>>
}
