#[derive(sqlx::FromRow, serde::Serialize, Debug)]
pub struct ProductInventoryRecord {
    pub id: uuid::Uuid,
    pub product_id: uuid::Uuid,
    pub inventory_id: uuid::Uuid,
    pub allocation: i32
}

#[derive(sqlx::FromRow, serde::Serialize)]
pub struct Inventory {
    pub id: uuid::Uuid,
    pub inventory_name: String,
    pub inventory_reference: String,
}
