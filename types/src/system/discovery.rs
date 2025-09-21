use crate::DeviceID;
use http::Uri;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Addresses {
    #[serde(default)]
    #[serde(with = " http_serde_ext::uri::vec")]
    pub addresses: Vec<Uri>,
}

pub type Discovery = HashMap<DeviceID, Addresses>;
