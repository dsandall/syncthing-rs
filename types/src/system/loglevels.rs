use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum LogLevel {
    Debug,
    Info,
    Warn,
    Error,
}

#[derive(Debug, Deserialize)]
pub struct LogLevelsInfo {
    pub levels: HashMap<String, LogLevel>,
    pub packages: HashMap<String, String>,
}
