use std::collections::HashMap;

#[derive(Debug)]
pub enum Event {
    AmountDeposited { trx_id: u32, amount: f64 },
    AmountWithdrawn { trx_id: u32, amount: f64 },
    DisputeRaised { trx_id: u32 },
    DisputeResolved { trx_id: u32 },
    ChargebackIssued { trx_id: u32 },
}
