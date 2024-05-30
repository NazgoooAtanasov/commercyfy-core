use serde::{Deserialize, Serialize};

use super::base_extensions::ObjectCustomFields;

#[derive(Deserialize)]
pub struct CreateInventory {
    pub inventory_reference: String,
    pub inventory_name: String,
    pub custom_fields: ObjectCustomFields,
}

impl CreateInventory {
    pub fn validate(&self) -> Result<(), String> {
        if self.inventory_name.is_empty() {
            return Err("'inventory_name' is mandatory field".to_string());
        }

        if self.inventory_reference.is_empty() {
            return Err("'inventory_reference' is mandatory field".to_string());
        }

        return Ok(());
    }
}

#[derive(Serialize, Deserialize)]
pub struct CreateInventoryRecord {
    pub product_id: uuid::Uuid,
    pub inventory_id: uuid::Uuid,
    pub allocation: i32,
}

impl CreateInventoryRecord {
    pub fn validate(&self) -> Result<(), String> {
        if self.product_id.to_string().is_empty() {
            return Err("'product_id' is mandatory".to_string());
        }

        if self.inventory_id.to_string().is_empty() {
            return Err("'inventory_id' is mandatory".to_string());
        }

        if self.allocation < 0 {
            return Err("'allocation' should not be negative".to_string());
        }

        return Ok(());
    }
}
