use crate::system::Entry;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Log {
    pub messages: Vec<Entry>,
}
