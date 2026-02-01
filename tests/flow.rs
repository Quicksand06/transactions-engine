#[cfg(test)]
mod tests {
    use trx_engine::{EventStore, ProjectionStore, process_csv_file};

    fn round4(v: f64) -> f64 {
        (v * 10_000.0).round() / 10_000.0
    }

    #[test]
    fn simple_deposit_withdraw_dispute_resolve_matches_expected_balances() {
        let path = "tests/samples/simple.csv";
        let mut es = EventStore::default();
        let mut ps = ProjectionStore::default();

        process_csv_file(path, &mut es, &mut ps).unwrap();

        let rows = ps.output();
        let account = rows.iter().find(|r| r.client_id == 101).unwrap();

        assert_eq!(round4(account.available), 7.4454);
        assert_eq!(round4(account.held), 0.0);
        assert_eq!(round4(account.total), 7.4454);
        assert_eq!(account.locked, false);
    }

    #[test]
    fn simple_deposit_withdraw_dispute_chargeback_gets_locked() {
        let path = "tests/samples/simple_chargeback.csv";

        let mut es = EventStore::default();
        let mut ps = ProjectionStore::default();

        process_csv_file(path, &mut es, &mut ps).unwrap();

        let rows = ps.output();
        let account = rows.iter().find(|r| r.client_id == 101).unwrap();

        assert_eq!(round4(account.available), 4.3452);
        assert_eq!(round4(account.held), 0.0);
        assert_eq!(round4(account.total), 4.3452);
        assert_eq!(account.locked, true);
    }

    #[test]
    fn dispute_without_resolve_matches_expected_balances() {
        let path = "tests/samples/dispute_without_resolve.csv";

        let mut es = EventStore::default();
        let mut ps = ProjectionStore::default();

        process_csv_file(path, &mut es, &mut ps).unwrap();

        let rows = ps.output();
        let account = rows.iter().find(|r| r.client_id == 101).unwrap();

        assert_eq!(round4(account.available), 2.6319);
        assert_eq!(round4(account.held), 3.4001);
        assert_eq!(round4(account.total), 6.032);
        assert_eq!(account.locked, false);
    }

    #[test]
    fn multiple_disputes_matches_expected_balances() {
        let path = "tests/samples/multiple_disputes.csv";

        let mut es = EventStore::default();
        let mut ps = ProjectionStore::default();

        process_csv_file(path, &mut es, &mut ps).unwrap();

        let rows = ps.output();
        let account = rows.iter().find(|r| r.client_id == 101).unwrap();

        assert_eq!(round4(account.available), 5.0);
        assert_eq!(round4(account.held), 1.0);
        assert_eq!(round4(account.total), 6.0);
        assert_eq!(account.locked, false);
    }

    #[test]
    fn no_trx_for_dispute_matches_expected_balances() {
        let path = "tests/samples/skip_dispute.csv";

        let mut es = EventStore::default();
        let mut ps = ProjectionStore::default();

        process_csv_file(path, &mut es, &mut ps).unwrap();

        let rows = ps.output();
        let account = rows.iter().find(|r| r.client_id == 101).unwrap();

        assert_eq!(round4(account.available), 9.0);
        assert_eq!(round4(account.held), 0.0);
        assert_eq!(round4(account.total), 9.0);
        assert_eq!(account.locked, false);
    }

    #[test]
    fn error_in_input_matches_expected_balances() {
        let path = "tests/samples/invalid.csv";

        let mut es = EventStore::default();
        let mut ps = ProjectionStore::default();

        process_csv_file(path, &mut es, &mut ps).unwrap();

        let rows = ps.output();
        let account = rows.iter().find(|r| r.client_id == 101).unwrap();

        assert_eq!(round4(account.available), 27.0);
        assert_eq!(round4(account.held), 0.0);
        assert_eq!(round4(account.total), 27.0);
        assert_eq!(account.locked, false);
    }
}
