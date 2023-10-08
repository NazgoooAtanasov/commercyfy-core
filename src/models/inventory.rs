use serde::{Serialize, Deserialize};
use tokio_postgres::Row;

#[derive(Serialize, Deserialize, Debug)]
pub struct ProductInventoryRecord {
    pub product_id: uuid::Uuid,
    pub allocation: i32
}

impl From<&Row> for ProductInventoryRecord {
    fn from(value: &Row) -> Self {
        return Self{
            product_id: value.get("product_id"),
            allocation: value.get("allocation")
        };
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Inventory {
    pub id: uuid::Uuid,
    pub inventory_name: String,
    pub inventory_reference: String,
    pub products: Vec<ProductInventoryRecord>
}

impl From<&Row> for Inventory {
    fn from(value: &Row) -> Self {
        return Self{
            id: value.get("inventory_id"),
            inventory_name: value.get("inventory_name"),
            inventory_reference: value.get("inventory_reference"),
            products: vec![]
        };
    }
}

impl From<&Vec<Row>> for Inventory {
    fn from(value: &Vec<Row>) -> Self {
        let first_row = value.get(0).unwrap();
        let mut inventory = Inventory::from(first_row);

        let product_records: Vec<ProductInventoryRecord> = value.iter().map(|x| ProductInventoryRecord::from(x)).collect();
        inventory.products = product_records;

        return inventory;
    }
}
