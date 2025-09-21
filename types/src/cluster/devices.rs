use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::net::SocketAddr;

use crate::{DeviceID, Timestamp};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ObservedDevice {
    pub time: Timestamp,
    pub name: String,
    pub adress: SocketAddr,
}

pub type PendingDevices = HashMap<DeviceID, ObservedDevice>;
