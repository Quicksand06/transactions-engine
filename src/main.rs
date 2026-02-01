use std::error::Error;
use std::{env, io};

use trx_engine::{EventStore, ProjectionStore, process_csv_file, write_accounts_csv};

fn main() -> Result<(), Box<dyn Error>> {
    let path = env::args()
        .nth(1)
        .ok_or("Please provide a path to the events file")?;

    let mut event_store = EventStore::default();
    let mut projection_store = ProjectionStore::default();

    process_csv_file(&path, &mut event_store, &mut projection_store)?;
    write_accounts_csv(io::stdout(), &projection_store)?;
    Ok(())
}
