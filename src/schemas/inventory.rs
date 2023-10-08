use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct CreateInventory {
    pub inventory_reference: String,
    pub inventory_name: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateInventoryRecord {
    pub product_id: uuid::Uuid,
    pub inventory_id: uuid::Uuid,
    pub allocation: i32
}
