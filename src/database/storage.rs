use std::path::{Path, PathBuf};

use anyhow::Result;

#[derive(Debug)]
pub struct Database {
    pub db_path: PathBuf,
}

impl Database {
    pub fn create(db_path: &str) -> Result<Database> {
        let path = Path::new(db_path);
        Ok(Database {
            db_path: path.to_owned(),
        })
    }

    pub fn open(db_path: &str) -> Result<Database> {
        let path = Path::new(db_path);
        Ok(Database {
            db_path: path.to_owned(),
        })
    }
}
