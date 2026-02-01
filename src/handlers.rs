use crate::commands::Command;
use crate::event_store::EventStore;
use crate::events::Event;
use crate::projections::{ProjectionStore, TransactionType};

pub struct CommandHandler;

impl CommandHandler {
    pub fn handle_command(
        event_store: &mut EventStore,
        projection_store: &mut ProjectionStore,
        cmd: Command,
    ) {
        match cmd {
            Command::Deposit {
                client_id,
                trx_id,
                amount,
            } => {
                let event = Event::AmountDeposited { amount };
                projection_store.update_for_client(client_id, &event);
                let _ = projection_store.insert_transaction(
                    client_id,
                    trx_id,
                    amount,
                    TransactionType::Deposit,
                );
                event_store.apply(client_id, event);
            }
            Command::Withdrawal {
                client_id,
                trx_id,
                amount,
            } => {
                let client = projection_store.get_client_account(client_id);
                // check if a client has enough funds
                if client.available < amount {
                    return;
                }

                let event = Event::AmountWithdrawn { amount };
                projection_store.update_for_client(client_id, &event);
                projection_store.insert_transaction(
                    client_id,
                    trx_id,
                    amount,
                    TransactionType::Withdrawal,
                );
                event_store.apply(client_id, event);
            }
            Command::Dispute { client_id, trx_id } => {
                let Some(trx) = projection_store.get_client_transaction(client_id, trx_id) else {
                    return;
                };

                let event = Event::DisputeRaised { amount: trx.amount };
                projection_store.update_for_client(client_id, &event);
                event_store.apply(client_id, event);
            }
            Command::Resolve { client_id, trx_id } => {
                let Some(trx) = projection_store.get_client_transaction(client_id, trx_id) else {
                    return;
                };

                let event = Event::DisputeResolved { amount: trx.amount };
                projection_store.update_for_client(client_id, &event);
                event_store.apply(client_id, event);
            }
            Command::Chargeback { client_id, trx_id } => {
                let Some(trx) = projection_store.get_client_transaction(client_id, trx_id) else {
                    return;
                };

                let event = Event::ChargebackIssued { amount: trx.amount };
                projection_store.update_for_client(client_id, &event);
                event_store.apply(client_id, event);
            }
        }
    }
}
