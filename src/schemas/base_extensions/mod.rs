use std::collections::HashMap;

use crate::models::base_extensions::FieldExtensionObject;

#[derive(serde::Deserialize)]
pub struct CreateBaseField {
    pub name: String,
    pub description: Option<String>,
    pub mandatory: bool,
}

#[derive(serde::Deserialize)]
pub struct CreateStringField {
    pub max_len: Option<i64>,
    pub min_len: Option<i64>,
}

#[derive(serde::Deserialize)]
#[serde(tag = "$type", rename_all = "lowercase")]
pub enum CreateCustomFieldEntry {
    STRING(CreateStringField),
    INT,
}

#[derive(serde::Deserialize)]
pub struct CreateCustomField {
    #[serde(rename = "$object")]
    pub object: FieldExtensionObject,

    #[serde(flatten)]
    pub base_felds: CreateBaseField,

    #[serde(flatten)]
    pub custom: CreateCustomFieldEntry,
}

impl CreateCustomField {
    pub fn validate(&self) -> Result<(), String> {
        if self.base_felds.name.is_empty() {
            return Err("'name' is mandatory field".to_string());
        }

        return Ok(());
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
#[serde(untagged)]
pub enum ObjectCustomField {
    STRING(String),
    INT(i64),
}

pub type ObjectCustomFields = Option<HashMap<String, ObjectCustomField>>;
