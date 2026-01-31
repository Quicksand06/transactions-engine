use crate::commands::Command;
use crate::event_store::EventStore;
use crate::events::Event;
use crate::projections::{ProjectionStore, TransactionType};
use csv::StringRecord;
use std::env;
use std::error::Error;
use std::fs::File;

mod commands;
mod event_store;
mod events;
mod projections;

fn main() -> Result<(), Box<dyn Error>> {
    let path = env::args()
        .nth(1)
        .expect("Please provide a path to the events file");

    let file = File::open(path)?;
    let mut reader = csv::ReaderBuilder::new()
        .trim(csv::Trim::All)
        .from_reader(file);

    let mut event_store = EventStore::default();
    let mut projection_store = ProjectionStore::default();
    for result in reader.records() {
        let record = match result {
            Ok(record) => record,
            Err(_) => continue,
        };

        if let Some(cmd) = parse_command(&record) {
            println!("{:?}", cmd);
            handle_command(&mut event_store, &mut projection_store, cmd);
        }
    }
    let output = projection_store.output();
    for o in output {
        println!("{}", o);
    }

    Ok(())
}

fn parse_command(record: &StringRecord) -> Option<Command> {
    let trx_type = record.get(0)?;
    let client_id: u16 = record.get(1)?.parse().ok()?;
    let trx_id: u32 = record.get(2)?.parse().ok()?;
    match trx_type {
        "deposit" => {
            let amount: f64 = record.get(3)?.parse().ok()?;
            Some(Command::Deposit {
                client_id,
                trx_id,
                amount,
            })
        }
        "withdrawal" => {
            let amount: f64 = record.get(3)?.parse().ok()?;
            Some(Command::Withdrawal {
                client_id,
                trx_id,
                amount,
            })
        }
        "dispute" => Some(Command::Dispute { client_id, trx_id }),
        "resolve" => Some(Command::Resolve { client_id, trx_id }),
        "chargeback" => Some(Command::Chargeback { client_id, trx_id }),
        _ => None, // unknown command, so ignore
    }
}

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
            let event = Event::AmountDeposited { trx_id, amount };
            projection_store.update_for_client(client_id, &event);
            let _ =
                projection_store.insert_transaction(client_id, trx_id,amount,TransactionType::Deposit);
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

            let event = Event::AmountWithdrawn { trx_id, amount };
            projection_store.update_for_client(client_id, &event);
            projection_store.insert_transaction(client_id, trx_id,amount, TransactionType::Withdrawal);
            event_store.apply(client_id, event);
        }
        Command::Dispute { client_id, trx_id } => {
            let trx = projection_store.get_client_transaction(client_id, trx_id);
            if trx.is_none() { return }

            let event = Event::DisputeRaised { trx_id };
            projection_store.update_for_client(client_id, &event);
            projection_store.insert_transaction(client_id, trx_id, 0.0, TransactionType::Dispute);
            event_store.apply(client_id, event);
        }
        Command::Resolve { client_id, trx_id } => {
            let trx = projection_store.get_client_transaction(client_id, trx_id);
            if trx.is_none() { return }

            let event = Event::DisputeResolved { trx_id };
            projection_store.update_for_client(client_id, &event);
            projection_store.insert_transaction(client_id, trx_id,0.0, TransactionType::Resolve);
            event_store.apply(client_id, event);
        }
        Command::Chargeback { client_id, trx_id } => {
            let trx = projection_store.get_client_transaction(client_id, trx_id);
            if trx.is_none() { return }

            let event = Event::ChargebackIssued { trx_id };
            projection_store.update_for_client(client_id, &event);
            projection_store.insert_transaction(client_id, trx_id, 0.0,TransactionType::Chargeback);
            event_store.apply(client_id, event);
        }
    }
}
