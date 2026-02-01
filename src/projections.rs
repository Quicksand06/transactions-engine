use crate::events::Event;
use std::collections::HashMap;

#[derive(Default, Debug)]
pub struct ClientAccount {
    pub total: f64,
    pub held: f64,
    pub available: f64,
    pub locked: bool,
}

// Mutations
impl ClientAccount {
    pub fn deposit(&mut self, amount: f64) {
        self.total += amount;
        self.available = self.total - self.held;
    }

    pub fn withdraw(&mut self, amount: f64) {
        self.total -= amount;
        self.available = self.total - self.held;
    }

    pub fn freeze(&mut self, amount: f64) {
        self.held += amount;
        self.available = self.total - self.held;
    }

    pub fn unfreeze(&mut self, amount: f64) {
        self.held -= amount;
        self.available = self.total - self.held;
    }

    pub fn lock(&mut self, amount: f64) {
        self.held -= amount;
        self.total -= amount;
        self.available = self.total - self.held;
        self.locked = true;
    }
}

#[derive(Debug, PartialEq)]
pub struct ClientAccountDto {
    pub client_id: u16,
    pub available: f64,
    pub held: f64,
    pub total: f64,
    pub locked: bool,
}

pub struct Transaction {
    pub amount: f64,
    pub trx_type: TransactionType,
}

pub enum TransactionType {
    Deposit,
    Withdrawal,
}

#[derive(Default)]
pub struct ProjectionStore {
    clients: HashMap<u16, ClientAccount>,
    client_transactions: HashMap<(u16, u32), Transaction>,
}

impl ProjectionStore {
    pub fn output(&self) -> Vec<ClientAccountDto> {
        self.clients
            .iter()
            .map(|(&client_id, account)| ClientAccountDto {
                client_id,
                available: account.available,
                held: account.held,
                total: account.total,
                locked: account.locked,
            })
            .collect()
    }

    // No validations here, apply event as a statement of fact
    pub fn update_for_client(&mut self, client_id: u16, event: &Event) {
        if self.clients.get(&client_id).is_none() {
            self.clients.insert(client_id, ClientAccount::default());
        }
        match event {
            Event::AmountDeposited { amount } => {
                let client = self.clients.get_mut(&client_id).unwrap();
                client.deposit(*amount);
            }
            Event::AmountWithdrawn { amount } => {
                let client = self.clients.get_mut(&client_id).unwrap();
                client.withdraw(*amount);
            }
            Event::DisputeRaised { amount } => {
                let client = self.clients.get_mut(&client_id).unwrap();
                client.freeze(*amount);
            }
            Event::DisputeResolved { amount } => {
                let client = self.clients.get_mut(&client_id).unwrap();
                client.unfreeze(*amount);
            }
            Event::ChargebackIssued { amount } => {
                let client = self.clients.get_mut(&client_id).unwrap();
                client.lock(*amount);
            }
        }
    }

    pub fn insert_transaction(
        &mut self,
        client_id: u16,
        trx_id: u32,
        amount: f64,
        trx_type: TransactionType,
    ) {
        use std::collections::hash_map::Entry;

        match self.client_transactions.entry((client_id, trx_id)) {
            Entry::Vacant(v) => {
                v.insert(Transaction { amount, trx_type });
            }
            Entry::Occupied(_) => {
                // duplicate tx: skip (by spec)
                eprintln!("Duplicate transaction: client={} tx={}", client_id, trx_id);
            }
        }
    }

    /// Returns a mutable reference to the client's account
    /// optionally create a new one on the fly
    pub fn get_client_account(&mut self, client_id: u16) -> &ClientAccount {
        self.clients
            .entry(client_id)
            .or_insert_with(ClientAccount::default)
    }

    pub fn get_client_transaction(&self, client_id: u16, trx_id: u32) -> Option<&Transaction> {
        self.client_transactions.get(&(client_id, trx_id))
    }
}
