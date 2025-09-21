use crate::DeviceID;
use http::Uri;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct Addresses {
    #[serde(with = " http_serde_ext::uri::vec")]
    pub addresses: Vec<Uri>,
}

pub type Discovery = HashMap<DeviceID, Addresses>;
