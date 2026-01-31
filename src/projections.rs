use crate::events::Event;
use std::collections::HashMap;

#[derive(Default)]
pub struct ClientAccount {
    pub total: f64,
    pub held: f64,
    pub available: f64,
    pub locked: bool,
}

pub struct Transaction {
    amount: f64,
    trx_type: TransactionType,
}

pub enum TransactionType {
    Deposit,
    Withdrawal,
    Dispute,
    Resolve,
    Chargeback,
}

#[derive(Default)]
pub struct ProjectionStore {
    clients: HashMap<u16, ClientAccount>,
    client_transactions: HashMap<(u16, u32), Transaction>,
}

impl ProjectionStore {
    pub fn output(&self) -> Vec<String> {
        self.clients
            .iter()
            .map(|(client_id, client)| {
                format!(
                    "{}, {}, {}, {}, {}",
                    client_id, client.available, client.held, client.total, client.locked
                )
            })
            .collect()
    }

    pub fn update_for_client(&mut self, client_id: u16, event: &Event) {
        if self.clients.get(&client_id).is_none() {
            self.clients.insert(client_id, ClientAccount::default());
        }
        match event {
            Event::AmountDeposited { trx_id, amount } => {
                let client = self.clients.get_mut(&client_id).unwrap();
                client.total += amount;
                client.available = client.total - client.held;
            }
            Event::AmountWithdrawn { trx_id, amount } => {
                let client = self.clients.get_mut(&client_id).unwrap();
                client.total -= amount;
                client.available = client.total - client.held;
            }
            Event::DisputeRaised { trx_id } => {
                let amount = self
                    .client_transactions
                    .get(&(client_id, *trx_id))
                    .unwrap()
                    .amount;
                let client = self.clients.get_mut(&client_id).unwrap();
                client.held += amount;
                client.available = client.total - client.held;
            }
            Event::DisputeResolved { trx_id } => {
                let amount = self
                    .client_transactions
                    .get(&(client_id, *trx_id))
                    .unwrap()
                    .amount;
                let client = self.clients.get_mut(&client_id).unwrap();
                client.held -= amount;
                client.available = client.total - client.held;
            }
            Event::ChargebackIssued { trx_id } => {
                let amount = self
                    .client_transactions
                    .get(&(client_id, *trx_id))
                    .unwrap()
                    .amount;
                let client = self.clients.get_mut(&client_id).unwrap();
                client.held = 0.0;
                client.total -= amount;
                client.available = client.total - client.held;
                client.locked = true;
            }
        }
    }

    pub fn insert_transaction(
        &mut self,
        client_id: u16,
        trx_id: u32,
        amount: f64,
        trx_type: TransactionType,
    ) -> bool {
        self.client_transactions
            .insert(
                (client_id, trx_id),
                Transaction {
                    amount,
                    trx_type,
                },
            )
            .is_some()
    }

    pub fn get_client_account(&mut self, client_id: u16) -> &ClientAccount {
        self.clients
            .entry(client_id)
            .or_insert_with(ClientAccount::default)
    }

    pub fn get_client_transaction(&self, client_id: u16, trx_id: u32) -> Option<&Transaction> {
        self.client_transactions.get(&(client_id, trx_id))
    }
}
