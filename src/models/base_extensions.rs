use serde::Serialize;

#[derive(Serialize)]
pub struct MigrationGenerated {
    pub file_path: String
}

pub struct __MetaProductCustomField {
    pub id: uuid::Uuid,
    pub name: String,
    pub description: Option<String>,
    pub value_type: String,
    pub default_value: Option<String>,
    pub mandatory: bool
}
