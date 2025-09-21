use crate::system::Entry;
use serde::{Deserialize, Serialize};

#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Log {
    #[serde(default)]
    pub messages: Vec<Entry>,
}
