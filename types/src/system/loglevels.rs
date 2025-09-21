use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum LogLevel {
    Debug,
    Info,
    Warn,
    Error,
}

#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct LogLevelsInfo {
    pub levels: HashMap<String, LogLevel>,
    pub packages: HashMap<String, String>,
}
