use crate::{Client, Fallible};
use futures_core::future::BoxFuture;
use futures_core::ready;
use futures_core::stream::Stream;
use std::pin::Pin;
use std::task::{Context, Poll};
use syncthing_types::events::{Event, EventType};

pub struct EventClient {
    client: Client,
    events: Vec<EventType>,
}

impl EventClient {
    fn receive(self, since: Option<u64>) -> BoxFuture<'static, (Self, Fallible<Vec<Event>>)> {
        Box::pin(async move {
            let data = self.client.get_events(since, None, &self.events).await;
            (self, data)
        })
    }
}

enum State {
    Buffer(Option<EventClient>, Vec<Event>),
    Future(BoxFuture<'static, (EventClient, Fallible<Vec<Event>>)>),
}

//TODO:self correction mechanism see: https://docs.syncthing.net/rest/events-get.html#events-get
pub struct EventStream {
    state: State,
    since: Option<u64>,
}

impl EventStream {
    pub(crate) fn new(client: Client, events: Vec<EventType>) -> Self {
        let event_client = EventClient { client, events };
        Self {
            state: State::Future(event_client.receive(None)),
            since: None,
        }
    }
}

impl Stream for EventStream {
    type Item = Fallible<Event>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        match &mut self.state {
            State::Future(fut) => match ready!(fut.as_mut().poll(cx)) {
                (event_client, Ok(mut data)) => {
                    data.reverse();
                    if let Some(event) = data.pop() {
                        self.state = State::Buffer(Some(event_client), data);
                        self.since = Some(event.id);
                        Poll::Ready(Some(Ok(event)))
                    } else {
                        self.state = State::Future(event_client.receive(self.since));
                        Poll::Pending
                    }
                }
                (event_client, Err(err)) => {
                    self.state = State::Future(event_client.receive(self.since));
                    Poll::Ready(Some(Err(err)))
                }
            },
            State::Buffer(connection_events, data) => {
                if let Some(event) = data.pop() {
                    self.since = Some(event.id);
                    Poll::Ready(Some(Ok(event)))
                } else {
                    let event_client = connection_events.take().unwrap();
                    self.state = State::Future(event_client.receive(self.since));
                    Poll::Pending
                }
            }
        }
    }
}
