#[derive(serde::Serialize, serde::Deserialize, Debug)]
#[serde(untagged)]
pub enum UnstructuredEntryType {
    STRING(String),
    INT(i64),
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct UnstructuredEntry {
    pub extr_ref: String,
    pub field_name: String,
    pub value: UnstructuredEntryType,
}
