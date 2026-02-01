use std::error::Error;
use std::fs::File;
use std::io::{self, Read, Write};

use crate::{CommandHandler, CommandParser, EventStore, ProjectionStore};

pub fn process_csv_file(
    path: &str,
    event_store: &mut EventStore,
    projection_store: &mut ProjectionStore,
) -> Result<(), Box<dyn Error>> {
    let file = File::open(path)?;
    process_csv_reader(file, event_store, projection_store)
}

pub fn process_csv_reader<R: Read>(
    reader: R,
    event_store: &mut EventStore,
    projection_store: &mut ProjectionStore,
) -> Result<(), Box<dyn Error>> {
    let mut csv = csv::ReaderBuilder::new()
        .trim(csv::Trim::All)
        .flexible(true)
        .from_reader(reader);

    for (idx, result) in csv.records().enumerate() {
        let record = match result {
            Ok(r) => r,
            Err(e) => {
                eprintln!("CSV parse error at row {}: {}", idx + 1, e);
                continue;
            }
        };

        let Some(cmd) = CommandParser::parse_command(&record) else {
            continue;
        };

        CommandHandler::handle_command(event_store, projection_store, cmd);
    }

    Ok(())
}

pub fn write_accounts_csv<W: Write>(
    writer: W,
    projection_store: &ProjectionStore,
) -> Result<(), Box<dyn Error>> {
    let mut out = io::BufWriter::new(writer);

    writeln!(out, "client,available,held,total,locked")?;

    let rows = projection_store.output();

    for r in rows {
        writeln!(
            out,
            "{},{:.4},{:.4},{:.4},{}",
            r.client_id, r.available, r.held, r.total, r.locked
        )?;
    }
    
    Ok(())
}
