mod client;
mod events;
#[cfg(test)]
mod tests;

pub use client::Client;

pub type Fallible<T> = Result<T, anyhow::Error>;
