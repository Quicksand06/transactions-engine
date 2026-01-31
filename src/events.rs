use std::collections::HashMap;

#[derive(Debug)]
pub enum Event {
    AmountDeposited {
        client_id: u16,
        trx_id: u32,
        amount: f64,
    },
    AmountWithdrawn {
        client_id: u16,
        trx_id: u32,
        amount: f64,
    },
    DisputeRaised {
        client_id: u16,
        trx_id: u32,
    },
    DisputeResolved {
        client_id: u16,
        trx_id: u32,
    },
    ChargebackIssued {
        client_id: u16,
        trx_id: u32,
    },
}

impl Event {
    pub fn client_id(&self) -> u16 {
        match *self {
            Event::AmountDeposited { client_id, .. } => client_id,
            Event::AmountWithdrawn { client_id, .. } => client_id,
            Event::DisputeRaised { client_id, .. } => client_id,
            Event::DisputeResolved { client_id, .. } => client_id,
            Event::ChargebackIssued { client_id, .. } => client_id,
        }
    }
}

#[derive(Default)]
pub struct EventStore {
    /// streams are represented as a map of client_id (unique) to a list of events
    pub streams: HashMap<u16, Vec<Event>>,
}

impl EventStore {

    /// Appends an event to the client's stream
    /// If no stream for the client, initialize the steam
    pub fn apply(&mut self, event: Event) {
        self.streams
            .entry(event.client_id())
            .or_default()
            .push(event);
    }

    /// Returns the stream of events for the given client
    pub fn stream(&self, client_id: u16) -> &[Event] {
        self.streams
            .get(&client_id)
            .map(|v| v.as_slice())
            .unwrap_or(&[])
    }
}
