use rust_decimal::Decimal;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Pricebook {
    pub id: uuid::Uuid,
    pub pricebook_name: String,
    pub pricebook_reference: String,
    pub pricebook_currency_code: String,
}

#[derive(Serialize, Deserialize)]
pub struct PricebookRecord {
    pub product_id: uuid::Uuid,
    pub price: Decimal,
}
