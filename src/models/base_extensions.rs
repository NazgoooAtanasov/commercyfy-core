#[derive(serde::Serialize, serde::Deserialize, sqlx::Type, Clone)]
#[serde(rename_all = "lowercase")]
#[sqlx(type_name = "metadataobjecttype")]
pub enum FieldExtensionObject {
    PRODUCT,
    CATEGORY,
    INVENTORY,
    PRICEBOOK,
}

#[derive(serde::Serialize, sqlx::Type)]
#[serde(rename_all = "lowercase")]
#[sqlx(type_name = "metadatafieldtype")]
pub enum FieldExtensionType {
    STRING,
    INT,
}

#[derive(serde::Serialize, sqlx::FromRow)]
pub struct FieldExtension {
    pub id: uuid::Uuid,

    #[serde(rename = "$object")]
    pub object: FieldExtensionObject,

    #[serde(rename = "$type")]
    pub r#type: FieldExtensionType,

    pub name: String,

    pub mandatory: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    // could only available if the extension field is of type STRING(FieldExtensionType::STRING)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_len: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_len: Option<i64>,
}
