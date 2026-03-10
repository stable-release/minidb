use clap::Parser;

use minidb::{database::database::Database, parser::startup::{self, StartupCli}};

pub fn main() -> anyhow::Result<()> {
    let startup = StartupCli::parse();

    let db_path: String = startup.db_name.unwrap_or("default".to_owned());
    let mut db = if startup.new {
        // Create new
        Database::create(&db_path)?;
    } else {
        // Retrieve db
        Database::open(&db_path)?;
    };



    Ok(())
}
