pub struct QueryChars(bool);

impl Default for QueryChars {
    fn default() -> Self {
        Self(false)
    }
}

impl QueryChars {
    pub fn next_char(&mut self) -> char {
        if self.0 {
            '&'
        } else {
            self.0 = true;
            '?'
        }
    }
}

use serde::{Deserialize, Deserializer};

pub fn default_on_null<'de, D, T: Default + Deserialize<'de>>(
    deserializer: D,
) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
{
    let opt = Option::deserialize(deserializer)?;
    Ok(opt.unwrap_or_default())
}

pub fn construct_event_url(
    since: Option<u64>,
    limit: Option<u64>,
    events: impl AsRef<[crate::events::EventType]>,
) -> serde_json::Result<String> {
    let mut path_and_query = crate::routes::EVENTS.to_owned();
    let events = events.as_ref();
    let mut query_chars = QueryChars::default();
    if !events.is_empty() {
        let events = serde_json::to_string(&events)?
            .chars()
            .filter(|e| !matches!(e, '\"' | '[' | ']'))
            .collect::<String>();
        path_and_query.push(query_chars.next_char());
        path_and_query.push_str("events=");
        path_and_query.push_str(events.as_ref());
    }
    if let Some(since) = since {
        path_and_query.push(query_chars.next_char());
        path_and_query.push_str("since=");
        path_and_query.push_str(since.to_string().as_ref());
    }
    if let Some(limit) = limit {
        path_and_query.push(query_chars.next_char());
        path_and_query.push_str("limit=");
        path_and_query.push_str(limit.to_string().as_ref());
    }
    Ok(path_and_query)
}
