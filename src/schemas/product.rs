use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateProductImage {
    pub src: String,
    pub srcset: Option<String>,
    pub alt: Option<String>
}

impl CreateProductImage {
    pub fn validate(&self) -> Result<(), String> {
        if self.src.is_empty() {
            return Err("The 'src' field is mandatory".to_string());
        }
        return Ok(());
    } 
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
    pub category_assignments: Option<Vec<uuid::Uuid>>,
    // pub custom_fields: Option<HashMap<String, CustomField>>
}

impl CreateProduct {
    pub fn validate(&self) -> Result<(), String> {
        if self.product_name.is_empty() {
            return Err("'product_name' is mandatory field".to_string());
        }

        if self.product_description.is_empty() {
            return Err("'product_description' is mandatory field".to_string());
        }

        return Ok(());
    }
}
