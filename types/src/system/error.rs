use crate::system::Entry;
use crate::utils;
use serde::{Deserialize, Serialize};

#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Error {
    #[serde(deserialize_with = "utils::default_on_null")]
    pub errors: Vec<Entry>,
}
