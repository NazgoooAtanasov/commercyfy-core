use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateCategory {
    pub category_name: String,
    pub category_description: Option<String>,
    pub category_reference: String
}

