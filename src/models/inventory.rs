use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ProductInventoryRecord {
    pub product_id: uuid::Uuid,
    pub allocation: i32
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Inventory {
    pub id: uuid::Uuid,
    pub inventory_name: String,
    pub inventory_reference: String,
    pub products: Vec<ProductInventoryRecord>
}
