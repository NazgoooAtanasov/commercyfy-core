use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::Peekable;
use std::path::Path;
use std::str::Chars;

use axum::{extract::State, Json};

use commercyfy_core::route_utils::CommercyfyResponse;
use commercyfy_core::{commercyfy_fail, commercyfy_success};
use commercyfy_core::services::logger::Logger;
use commercyfy_core::schemas::logs::CreateLog;

use super::{CommercyfyExtrState};

#[derive(serde::Serialize)]
pub struct EmptyResponse {}

#[derive(serde::Serialize, Debug)]
pub struct LogView {
    file: String,
    timestamp: String,
    level: String,
    message: String,
}

fn parse_log_bracketed(chars: &mut Peekable<Chars>) -> Result<String, String> {
    match chars.next() {
        Some(opening_braket) => {
            if opening_braket != '[' {
                return Err("Incorrect log format".to_string());
            }
        }
        None => {
            return Err("Incorrect log format".to_string());
        }
    };

    let mut thing = String::new();
    let mut parse = true;

    while parse {
        let c = match chars.next() {
            Some(c) => c,
            None => return Err("Incorrect log format".to_string()),
        };

        thing.push(c);

        if let Some(x) = chars.peek() {
            parse = if *x == ']' { false } else { true };
        }
    }

    match chars.next() {
        Some(closing_braket) => {
            if closing_braket != ']' {
                return Err("Incorrect log format".to_string());
            }
        }
        None => {
            return Err("Incorrect log format".to_string());
        }
    };

    return Ok(thing);
}

fn parse_log_message(chars: &mut Peekable<Chars>) -> Result<String, String> {
    match chars.next() {
        Some(col) => {
            if col != ':' {
                return Err("Incorrect log format".to_string());
            }
        }
        None => {
            return Err("Incorrect log format".to_string());
        }
    };

    match chars.next() {
        Some(space) => {
            if space != ' ' {
                return Err("Incorrect log format".to_string());
            }
        }
        None => {
            return Err("Incorrect log format".to_string());
        }
    };

    let mut message = String::new();

    while let Some(_) = chars.peek() {
        let c = match chars.next() {
            Some(c) => c,
            None => return Err("Incorrect log format".to_string()),
        };

        message.push(c);
    }

    return Ok(message);
}

/*
 * log message structure
 *
 * z -> [date][level]: message
 * date -> date_char {date}
 * date_char -> 0-9 | - | \space | :
 * level -> INFO | WARN | ERROR
 * message -> string {\space string}
 * string -> a-z | 0-9 | any
 */
fn parse_log(file: &str, line: &str) -> LogView {
    let mut chars = line.chars().peekable();

    let timestamp = parse_log_bracketed(&mut chars);
    let level = parse_log_bracketed(&mut chars);
    let message = parse_log_message(&mut chars);

    return LogView {
        file: file.to_string(),
        timestamp: timestamp.unwrap(),
        level: level.unwrap(),
        message: message.unwrap(),
    };
}

pub async fn get_logs() -> CommercyfyResponse<Vec<LogView>> {
    let mut logs: Vec<LogView> = vec![];
    let path = Path::new("./logs");

    let entries = match path.read_dir() {
        Ok(x) => x.into_iter().filter(|x| x.is_ok()).map(|x| x.unwrap()),
        Err(err) => return commercyfy_fail!(err.to_string()),
    };

    for entry in entries {
        if let Ok(fd) = File::open(entry.path()) {
            let reader = BufReader::new(fd);

            for line in reader.lines() {
                if let Ok(line) = line {
                    logs.push(parse_log(entry.path().to_str().unwrap(), &line));
                }
            }
        }
    }

    return commercyfy_success!(logs);
}

pub async fn create_log(
    State(state): CommercyfyExtrState,
    Json(payload): Json<CreateLog>,
) -> CommercyfyResponse<EmptyResponse> {
    let has_file = payload.file.is_some();
    let has_category = payload.category.is_some();

    if has_file && has_category {
        let _ = state.logger.file_category_log(
            payload.level,
            payload.file.unwrap().as_str(),
            payload.category.unwrap().as_str(),
            payload.message.as_str(),
        );
    } else if has_file {
        let _ = state.logger.file_category_log(
            payload.level,
            payload.file.unwrap().as_str(),
            "default",
            payload.message.as_str(),
        );
    } else if has_category {
        let _ = state.logger.category_log(
            payload.level,
            payload.category.unwrap().as_str(),
            payload.message.as_str(),
        );
    } else {
        let _ = state.logger.log(payload.level, payload.message.as_str());
    }

    return commercyfy_success!(EmptyResponse {});
}
