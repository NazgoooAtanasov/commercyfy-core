use serde::Serialize;

#[derive(Serialize)]
pub struct MigrationGenerated {
    pub file_path: String
}
