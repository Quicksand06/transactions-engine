#[derive(Debug)]
pub enum Command {
    Deposit { client_id: u16, trx_id: u32, amount: f64 },
    Withdrawal { client_id: u16, trx_id: u32, amount: f64 },
    Dispute { client_id: u16, trx_id: u32 },
    Resolve { client_id: u16, trx_id: u32 },
    Chargeback { client_id: u16, trx_id: u32 },
}
