use serde::{Deserialize, Serialize};

#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all(deserialize = "lowercase"))]
pub enum Pong {
    Pong,
}

#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Ping {
    pub ping: Pong,
}
