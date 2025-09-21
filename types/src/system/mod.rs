mod connections;
mod discovery;
mod error;
mod log;
mod loglevels;
mod ping;
mod upgrade;
mod version;

pub use connections::*;
pub use discovery::*;
pub use error::*;
pub use log::*;
pub use loglevels::*;
pub use ping::*;
pub use upgrade::*;
pub use version::*;

use serde::Deserialize;

use crate::Timestamp;

#[derive(Debug, Deserialize)]
pub struct Entry {
    pub when: Timestamp,
    pub message: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Statistics {
    pub at: Timestamp,
    pub in_bytes_total: u64,
    pub out_bytes_total: u64,
    pub started_at: Timestamp,
}
