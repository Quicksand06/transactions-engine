/// Event is a fact that smth has happened with the account
#[derive(Debug)]
pub enum Event {
    AmountDeposited { amount: f64 },
    AmountWithdrawn { amount: f64 },
    DisputeRaised { amount: f64 },
    DisputeResolved { amount: f64 },
    ChargebackIssued { amount: f64 },
}
