use crate::services::logger::LogLevel;

#[derive(serde::Deserialize)]
pub struct CreateLog {
    pub level: LogLevel,
    pub message: String,
    pub category: Option<String>,
    pub file: Option<String>,
}
