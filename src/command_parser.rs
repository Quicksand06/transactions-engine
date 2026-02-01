use crate::Command;
use csv::StringRecord;

pub struct CommandParser;

impl CommandParser {
    pub fn parse_command(record: &StringRecord) -> Option<Command> {
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
}
