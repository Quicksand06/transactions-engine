use crate::events::Event;
use std::collections::HashMap;

#[derive(Default)]
pub struct EventStore {
    /// streams are represented as a map of client_id (unique) to a list of events
    streams: HashMap<u16, Vec<Event>>,
}

impl EventStore {
    /// Appends an event to the client's stream
    /// If no stream for the client, initialize the steam
    pub fn apply(&mut self, client_id: u16, event: Event) {
        self.streams.entry(client_id).or_default().push(event);
    }

    /// Returns the stream of events for the given client
    pub fn stream(&self, client_id: u16) -> &[Event] {
        self.streams
            .get(&client_id)
            .map(|v| v.as_slice())
            .unwrap_or(&[])
    }
}
