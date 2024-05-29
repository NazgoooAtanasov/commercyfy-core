#[derive(sqlx::FromRow, serde::Serialize)]
pub struct Pricebook {
    pub id: uuid::Uuid,
    pub pricebook_name: String,
    pub pricebook_reference: String,
    pub pricebook_currency_code: String,
}

#[derive(serde::Serialize, sqlx::FromRow)]
pub struct PricebookRecord {
    pub id: uuid::Uuid,
    pub pricebook_id: uuid::Uuid,
    pub product_id: uuid::Uuid,
    pub price: rust_decimal::Decimal,
}
