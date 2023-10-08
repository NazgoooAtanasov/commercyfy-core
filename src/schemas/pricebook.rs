use rust_decimal::Decimal;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CreatePricebook {
    pub pricebook_name: String,
    pub pricebook_reference: String,
    pub pricebook_currency_code: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreatePricebookRecord {
    pub product_id: uuid::Uuid,
    pub price: Decimal,
}
