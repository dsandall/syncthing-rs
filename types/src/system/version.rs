use serde::{Deserialize, Serialize};

#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct VersionInfo {
    pub arch: String, //FIXME:use enum
    pub long_version: String,
    pub os: String, //FIXME:use enum
    pub version: String,
}
