pub mod account;
pub mod base_extensions;
pub mod category;
pub mod inventory;
pub mod logs;
pub mod portal_user;
pub mod pricebook;
pub mod product;

pub trait ValidationSchema {
    fn string_empty(&self, value: &str) -> bool {
        if value.len() <= 0 {
            return true;
        }
        return false;
    }

    fn validate(&self) -> Result<(), String>;
}
