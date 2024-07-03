use commercyfy_core::{
    models::base_extensions::FieldExtensionObject, schemas::base_extensions::{ObjectCustomField, ObjectCustomFields}, services::{
        db::DbService,
        unstructureddb::{entry::{UnstructuredEntry, UnstructuredEntryType}, UnstructuredDb},
    }
};
use crate::routes::CommercyfyState;
use std::sync::Arc;

pub async fn create_custom_fields(
    state: Arc<CommercyfyState>,
    extr_ref: String,
    object: FieldExtensionObject,
    custom_fields: &ObjectCustomFields,
) -> Result<(), String> {
    if let Some(custom_fields) = custom_fields {
        if custom_fields.is_empty() {
            return Ok(());
        }

        let mut unstructured_entries: Vec<UnstructuredEntry> = vec![];
        for (key, value) in custom_fields {
            let custom_field = match state.db_service.get_custom_field(object.clone(), key).await {
                Ok(custom_field) => custom_field,
                Err(err) => return Err(err.to_string()),
            };

            if let None = custom_field {
                return Err(format!("Custom field with name '{}', does not exist", key));
            }

            let custom_value = match value {
                ObjectCustomField::STRING(string) => {
                    UnstructuredEntryType::STRING(string.to_owned())
                }
                ObjectCustomField::INT(integer) => UnstructuredEntryType::INT(integer.to_owned()),
            };

            unstructured_entries.push(UnstructuredEntry {
                extr_ref: extr_ref.clone(),
                field_name: key.to_string(),
                value: custom_value,
            });
        }

        if let Err(err) = state
            .unstructureddb
            .put_custom_fields(object, unstructured_entries)
            .await
        {
            return Err(err.to_string());
        }
    }

    return Ok(());
}
