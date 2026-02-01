#[cfg(test)]
mod tests {
    use std::fs::File;
    use trx_engine::{CommandHandler, CsvParser, EventStore, ProjectionStore};

    fn run_engine_from_csv_path(path: &str) -> ProjectionStore {
        let file = File::open(path).expect("cannot open CSV sample file");

        let mut reader = csv::ReaderBuilder::new()
            .trim(csv::Trim::All)
            .flexible(true)
            .from_reader(file);

        let mut event_store = EventStore::default();
        let mut projection_store = ProjectionStore::default();

        for result in reader.records() {
            let record = match result {
                Ok(r) => r,
                Err(e) => {
                    eprintln!("CSV parse error: {e}");
                    continue;
                }
            };

            if let Some(cmd) = CsvParser::parse_command(&record) {
                CommandHandler::handle_command(&mut event_store, &mut projection_store, cmd);
            }
        }

        projection_store
    }

    /*fn assert_f64_close(actual: f64, expected: f64, eps: f64) {
        let diff = (actual - expected).abs();
        assert!(
            diff <= eps,
            "expected {expected}, got {actual}, diff {diff} > eps {eps}"
        );
    }*/

    fn round4(v: f64) -> f64 {
        (v * 10_000.0).round() / 10_000.0
    }

    #[test]
    fn simple_deposit_withdraw_dispute_resolve_matches_expected_balances() {
        let path = "tests/samples/simple.csv";

        let projection_store = run_engine_from_csv_path(path);
        let output = projection_store.output();

        let row = output
            .iter()
            .find(|r| r.client_id == 101)
            .expect("expected client not in the output");

        assert_eq!(round4(row.available), 7.4454);
        assert_eq!(round4(row.held), 0.0);
        assert_eq!(round4(row.total), 7.4454);
        assert_eq!(row.locked, false);
    }

    #[test]
    fn simple_deposit_withdraw_dispute_chargeback_gets_locked() {
        let path = "tests/samples/simple_chargeback.csv";

        let projection_store = run_engine_from_csv_path(path);
        let output = projection_store.output();

        let row = output
            .iter()
            .find(|r| r.client_id == 101)
            .expect("expected client not in the output");

        assert_eq!(round4(row.available), 4.3452);
        assert_eq!(round4(row.held), 0.0);
        assert_eq!(round4(row.total), 4.3452);
        assert_eq!(row.locked, true);
    }

    #[test]
    fn dispute_without_resolve_matches_expected_balances() {
        let path = "tests/samples/dispute_without_resolve.csv";

        let projection_store = run_engine_from_csv_path(path);
        let output = projection_store.output();

        let row = output
            .iter()
            .find(|r| r.client_id == 101)
            .expect("expected client not in the output");

        assert_eq!(round4(row.available), 2.6319);
        assert_eq!(round4(row.held), 3.4001);
        assert_eq!(round4(row.total), 6.032);
        assert_eq!(row.locked, false);
    }

    #[test]
    fn multiple_disputes_matches_expected_balances() {
        let path = "tests/samples/multiple_disputes.csv";

        let projection_store = run_engine_from_csv_path(path);
        let output = projection_store.output();

        let row = output
            .iter()
            .find(|r| r.client_id == 101)
            .expect("expected client not in the output");

        assert_eq!(round4(row.available), 5.0);
        assert_eq!(round4(row.held), 1.0);
        assert_eq!(round4(row.total), 6.0);
        assert_eq!(row.locked, false);
    }

    #[test]
    fn no_trx_for_dispute_matches_expected_balances() {
        let path = "tests/samples/skip_dispute.csv";

        let projection_store = run_engine_from_csv_path(path);
        let output = projection_store.output();

        let row = output
            .iter()
            .find(|r| r.client_id == 101)
            .expect("expected client not in the output");

        assert_eq!(round4(row.available), 9.0);
        assert_eq!(round4(row.held), 0.0);
        assert_eq!(round4(row.total), 9.0);
        assert_eq!(row.locked, false);
    }

    #[test]
    fn error_in_input_matches_expected_balances() {
        let path = "tests/samples/invalid.csv";
    
        let projection_store = run_engine_from_csv_path(path);
        let output = projection_store.output();
    
        let row = output
            .iter()
            .find(|r| r.client_id == 101)
            .expect("expected client not in the output");
    
        assert_eq!(round4(row.available), 27.0);
        assert_eq!(round4(row.held), 0.0);
        assert_eq!(round4(row.total), 27.0);
        assert_eq!(row.locked, false);
    }
}
