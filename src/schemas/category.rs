use super::base_extensions::ObjectCustomFields;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct CreateCategory {
    pub category_name: String,
    pub category_description: Option<String>,
    pub category_reference: String,
    pub custom_fields: ObjectCustomFields,
}

impl CreateCategory {
    pub fn validate(&self) -> Result<(), String> {
        if self.category_reference.is_empty() {
            return Err("\"category_reference\" is a required field".to_string());
        }

        if self.category_name.is_empty() {
            return Err("\"category_name\" is a required field".to_string());
        }

        return Ok(());
    }
}

#[derive(serde::Deserialize, Debug)]
pub struct AssignProductToCategory {
    pub product_ids: Vec<uuid::Uuid>,
    pub category_id: uuid::Uuid,
}

impl AssignProductToCategory {
    pub fn validate(&self) -> Result<(), String> {
        if self.product_ids.is_empty() {
            return Err("\"product_ids\" should contain at least one product id".to_string());
        }

        if self.category_id.to_string().is_empty() {
            return Err("\"category_id\" is a required field".to_string());
        }

        return Ok(());
    }
}
