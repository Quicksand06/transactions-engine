use std::env;
use std::error::Error;
use std::fs::File;

use trx_engine::{CommandHandler, CsvParser, EventStore, ProjectionStore};

fn main() -> Result<(), Box<dyn Error>> {
    let path = env::args()
        .nth(1)
        .expect("Please provide a path to the events file");

    let file = File::open(path)?;
    let mut reader = csv::ReaderBuilder::new()
        .trim(csv::Trim::All)
        .flexible(true) // allow variable-length records
        .from_reader(file);

    let mut event_store = EventStore::default();
    let mut projection_store = ProjectionStore::default();
    for result in reader.records() {
        let record = match result {
            Ok(record) => record,
            Err(e) => {
                eprintln!("CSV parse error: {e}");
                continue;
            }
        };

        if let Some(cmd) = CsvParser::parse_command(&record) {
            CommandHandler::handle_command(&mut event_store, &mut projection_store, cmd);
        }
    }

    println!("client,available,held,total,locked");
    for o in projection_store.output() {
        println!(
            "{},{:.4},{:.4},{:.4},{}",
            o.client_id, o.available, o.held, o.total, o.locked
        );
    }

    Ok(())
}
