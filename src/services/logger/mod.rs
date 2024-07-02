use std::{fs::OpenOptions, io::Write};

pub type LoggerResult = std::io::Result<()>;

pub trait Logger {
    fn log(&self, level: LogLevel, message: &str) -> LoggerResult;
    fn info(&self, message: &str) -> LoggerResult;
    fn warn(&self, message: &str) -> LoggerResult;
    fn error(&self, message: &str) -> LoggerResult;

    fn category_log(&self, level: LogLevel, category: &str, message: &str) -> LoggerResult;
    fn category_info(&self, category: &str, message: &str) -> LoggerResult;
    fn category_warn(&self, category: &str, message: &str) -> LoggerResult;
    fn category_error(&self, category: &str, message: &str) -> LoggerResult;

    fn file_category_log(
        &self,
        level: LogLevel,
        file: &str,
        category: &str,
        message: &str,
    ) -> LoggerResult;
    fn file_category_info(&self, file: &str, category: &str, message: &str) -> LoggerResult;
    fn file_category_warn(&self, file: &str, category: &str, message: &str) -> LoggerResult;
    fn file_category_error(&self, file: &str, category: &str, message: &str) -> LoggerResult;
}

pub struct GenericLogger {}
impl GenericLogger {
    pub fn new() -> Self {
        GenericLogger {}
    }

    pub fn get_file(&self, file: &str) -> std::io::Result<std::fs::File> {
        return OpenOptions::new()
            .write(true)
            .append(true)
            .create(true)
            .open(file);
    }

    fn format_log(&self, prefix: &str, message: &str) -> String {
        let time = chrono::Utc::now();
        return format!(
            "[{}][{}]: {}\n",
            time.format("%Y-%m-%d %H:%M:%S"),
            prefix,
            message
        );
    }

    fn write_to_file(&self, file: &str, category: &str, message: String) -> LoggerResult {
        if let Ok(mut file) =
            self.get_file(&format!("./logs/{}-{}.log", file, category.to_lowercase()))
        {
            let _ = file.write_all(message.as_bytes());
        }
        return Ok(());
    }
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LogLevel {
    INFO,
    WARN,
    ERROR,
}

impl Logger for GenericLogger {
    fn log(&self, level: LogLevel, message: &str) -> LoggerResult {
        return match level {
            LogLevel::INFO => self.info(message),
            LogLevel::WARN => self.warn(message),
            LogLevel::ERROR => self.error(message),
        };
    }

    fn info(&self, message: &str) -> LoggerResult {
        return self.category_info("default", message);
    }

    fn warn(&self, message: &str) -> LoggerResult {
        return self.category_warn("default", message);
    }

    fn error(&self, message: &str) -> LoggerResult {
        return self.category_error("default", message);
    }

    fn category_log(&self, level: LogLevel, category: &str, message: &str) -> LoggerResult {
        return match level {
            LogLevel::INFO => self.category_info(category, message),
            LogLevel::WARN => self.category_warn(category, message),
            LogLevel::ERROR => self.category_error(category, message),
        };
    }

    fn category_info(&self, category: &str, message: &str) -> LoggerResult {
        return self.file_category_info("commercyfy-core", category, message);
    }

    fn category_warn(&self, category: &str, message: &str) -> LoggerResult {
        return self.file_category_warn("commercyfy-core", category, message);
    }

    fn category_error(&self, category: &str, message: &str) -> LoggerResult {
        return self.file_category_error("commercyfy-core", category, message);
    }

    fn file_category_log(
        &self,
        level: LogLevel,
        file: &str,
        category: &str,
        message: &str,
    ) -> LoggerResult {
        return match level {
            LogLevel::INFO => self.file_category_info(file, category, message),
            LogLevel::WARN => self.file_category_warn(file, category, message),
            LogLevel::ERROR => self.file_category_error(file, category, message),
        };
    }

    fn file_category_info(&self, file: &str, category: &str, message: &str) -> LoggerResult {
        return self.write_to_file(file, category, self.format_log("INFO", message));
    }

    fn file_category_warn(&self, file: &str, category: &str, message: &str) -> LoggerResult {
        return self.write_to_file(file, category, self.format_log("WARN", message));
    }

    fn file_category_error(&self, file: &str, category: &str, message: &str) -> LoggerResult {
        return self.write_to_file(file, category, self.format_log("ERROR", message));
    }
}
