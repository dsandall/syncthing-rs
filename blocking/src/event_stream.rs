use crate::{Client, Fallible};
use std::iter::Iterator;
use syncthing_types::events::{Event, EventType};

//TODO:self correction mechanism see: https://docs.syncthing.net/rest/events-get.html#events-get
pub struct EventStream {
    client: Client,
    events: Vec<EventType>,
    buffer: Option<Vec<Event>>,
    since: Option<u64>,
}

impl EventStream {
    pub(crate) fn new(client: Client, events: Vec<EventType>) -> Self {
        Self {
            client,
            events,
            buffer: None,
            since: None,
        }
    }
}

impl Iterator for EventStream {
    type Item = Fallible<Event>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.buffer {
                Some(ref mut data) => {
                    if let Some(event) = data.pop() {
                        self.since = Some(event.id);
                        return Some(Ok(event));
                    } else {
                        self.buffer = None;
                    }
                }
                None => match self.client.get_events(self.since, None, &self.events) {
                    Ok(mut data) => {
                        data.reverse();
                        self.buffer = Some(data);
                    }
                    Err(err) => return Some(Err(err)),
                },
            }
        }
    }
}
