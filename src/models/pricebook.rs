use rust_decimal::Decimal;
use serde::{Serialize, Deserialize};
use tokio_postgres::Row;

#[derive(Debug, Serialize, Deserialize)]
pub struct Pricebook {
    pub id: uuid::Uuid,
    pub pricebook_name: String,
    pub pricebook_reference: String,
    pub pricebook_currency_code: String,
}

impl From<&Row> for Pricebook {
    fn from(row: &Row) -> Pricebook {
        return Pricebook {
            id: row.get("id"),
            pricebook_name: row.get("pricebook_name"),
            pricebook_reference: row.get("pricebook_reference"),
            pricebook_currency_code: row.get("pricebook_currency_code"),
        };
    }
}

#[derive(Serialize, Deserialize)]
pub struct PricebookRecord {
    pub product_id: uuid::Uuid,
    pub price: Decimal,
}

impl From<&Row> for PricebookRecord {
    fn from(value: &Row) -> Self {
        return PricebookRecord {
            product_id: value.get("product_id"),
            price: value.get("price"),
        };
    }
}
