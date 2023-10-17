use serde::Deserialize;

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Field<T> {
    pub name: String,
    pub description: Option<String>,
    pub mandatory: bool,
    pub default_value: Option<T>,
}

#[derive(Deserialize, Clone)]
#[serde(tag = "$type")]
pub enum CreateExtensionFieldVariant {
    #[serde(alias = "string", rename_all = "camelCase")]
    STRING(Field<String>),

    #[serde(alias = "boolean", rename_all = "camelCase")]
    BOOLEAN(Field<bool>),
}

#[derive(Deserialize, Clone)]
pub struct CreateExtensionField {
    #[serde(flatten)]
    pub variant: CreateExtensionFieldVariant,
}

#[derive(Deserialize, Clone, Copy)]
#[serde(rename_all = "camelCase")]
pub enum ExtensionType {
    Product,
}

#[derive(Deserialize)]
pub struct CreateExtension {
    #[serde(rename(deserialize = "$type"))]
    pub r#type: ExtensionType,
    pub fields: Vec<CreateExtensionField>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateMigrationUpdate {
    pub file_path: String
}
