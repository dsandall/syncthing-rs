use crate::system::Statistics;
use crate::{DeviceID, Timestamp};
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TotalConnectionsStatistic {
    pub at: Timestamp,
    pub in_bytes_total: u64,
    pub out_bytes_total: u64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConnectionInfo {
    pub address: String,
    #[serde(rename = "type")]
    pub connection_type: String,
    pub is_local: bool,
    pub crypto: String,

    #[serde(flatten)]
    pub statistics: Statistics,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConnectionStats {
    pub connected: bool,
    pub paused: bool,
    pub client_version: String,

    pub primary: Option<ConnectionInfo>,
    #[serde(default)]
    pub secondary: Vec<ConnectionInfo>,

    /// Total for primary + secondaries
    #[serde(flatten)]
    pub statistics: Statistics,
}

#[derive(Debug, Deserialize)]
pub struct Connections {
    pub total: TotalConnectionsStatistic,
    pub connections: HashMap<DeviceID, ConnectionStats>,
}
