use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use super::base_extensions::ObjectCustomFields;

#[derive(Serialize, Deserialize, Debug)]
pub struct CreatePricebook {
    pub pricebook_name: String,
    pub pricebook_reference: String,
    pub pricebook_currency_code: String,
    pub custom_fields: ObjectCustomFields,
}

impl CreatePricebook {
    pub fn validate(&self) -> Result<(), String> {
        if self.pricebook_name.is_empty() {
            return Err("'pricebook_name' is a mandatory field.".to_string());
        }

        if self.pricebook_reference.is_empty() {
            return Err("'pricebook_reference' is a mandatory field.".to_string());
        }

        if self.pricebook_currency_code.is_empty() {
            return Err("'pricebook_currency_code' is a mandatory field.".to_string());
        }

        return Ok(());
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreatePricebookRecord {
    pub pricebook_id: String,
    pub product_id: String,
    pub price: Decimal,
}

impl CreatePricebookRecord {
    pub fn validate(&self) -> Result<(), String> {
        if self.pricebook_id.is_empty() {
            return Err("'pricebook_id' is a mandatory field".to_string());
        }

        if self.product_id.is_empty() {
            return Err("'product_id' is a mandatory field".to_string());
        }

        if self.price.is_sign_negative() {
            return Err("'price' should not be negative".to_string());
        }

        return Ok(());
    }
}
