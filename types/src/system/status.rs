use crate::{DeviceID, Timestamp};
use http::Uri;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DiscoveryStatus {
    #[serde(rename = "discoveryEnabled")]
    pub enabled: bool,
    #[serde(rename = "discoveryStatus")]
    pub status: HashMap<String, String>,
}

#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ListenStatusEntry {
    pub error: Option<String>,
    #[serde(default)]
    #[serde(with = " http_serde_ext::uri::vec")]
    pub lan_addresses: Vec<Uri>,
    #[serde(default)]
    #[serde(with = " http_serde_ext::uri::vec")]
    pub wan_addresses: Vec<Uri>,
}

#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ConnectionStatusEntry {
    pub when: Timestamp,
    pub error: Option<String>,
}

#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Status {
    #[serde(rename = "myID")]
    pub device_id: DeviceID,
    pub goroutines: u32,
    pub alloc: u32,
    pub sys: u32,
    pub tilde: PathBuf,
    #[serde(flatten)]
    pub discovery: Option<DiscoveryStatus>,
    #[serde(default)]
    #[serde(with = "http_serde_ext::uri::hash_map_key")]
    pub connection_service_status: HashMap<Uri, ListenStatusEntry>,
    #[serde(default)]
    #[serde(with = "http_serde_ext::uri::hash_map_key")]
    pub last_dial_status: HashMap<Uri, ConnectionStatusEntry>,
    pub path_separator: String,
    pub ur_version_max: u32,
    pub uptime: u32,
    pub start_time: Timestamp,
    pub gui_address_overridden: bool,
    #[serde(with = "http_serde_ext::uri")]
    pub gui_address_used: Uri,
}
