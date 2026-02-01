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

    fn assert_f64_close(actual: f64, expected: f64, eps: f64) {
        let diff = (actual - expected).abs();
        assert!(
            diff <= eps,
            "expected {expected}, got {actual}, diff {diff} > eps {eps}"
        );
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

        assert_f64_close(row.available, 7.4454, 1e-9);
        assert_f64_close(row.held, 0.0, 1e-9);
        assert_f64_close(row.total, 7.4454, 1e-9);
        assert_eq!(row.locked, false);
    }
}
