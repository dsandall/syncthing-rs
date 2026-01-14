use crate::{DeviceID, FolderID};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Folder {
    pub id: FolderID,
    pub label: String,
    pub path: String,
    pub paused: bool,
    //#[serde(deserialize_with = "deserialize_device_ids")]
    pub devices: Vec<FolderDevice>,
}

#[derive(Deserialize, Debug)]
struct FolderDevice {
    #[serde(rename = "deviceID")]
    device_id: DeviceID,
}

//use serde::Deserializer;
//fn deserialize_device_ids<'de, D>(deserializer: D) -> Result<Vec<DeviceID>, D::Error>
//where
//    D: Deserializer<'de>,
//{
//    let devices = Vec::<FolderDevice>::deserialize(deserializer)?;
//    // reduce into only device IDs
//    Ok(devices.into_iter().map(|d| d.device_id).collect())
//}
