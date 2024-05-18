use serde::Serialize;
use tokio_postgres::Row;

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


impl From<Row> for __MetaProductCustomField {
    fn from(value: Row) -> Self {
        return Self {
            id: value.get("id"),
            name: value.get("name"),
            description: value.try_get("description").map_or(None, |x| Some(x)),
            value_type: value.get("value_type"),
            default_value: value.try_get("default_value").map_or(None, |x| Some(x)),
            mandatory: value.get("mandatory"),
        };
    }
}
