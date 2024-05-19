use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateCategory {
    pub category_name: String,
    pub category_description: Option<String>,
    pub category_reference: String
}

impl CreateCategory {
    pub fn validate(&self) -> Result<(), String> {
        if self.category_reference.is_empty() {
            return Err("\"category_reference\" is a required field".to_string());
        }

        if self.category_name.is_empty() {
            return Err("\"category_name\" is a required field".to_string())
        }

        return Ok(());
    }
}
