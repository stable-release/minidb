use std::{
    fs::{File, OpenOptions},
    path::{Path, PathBuf},
};

use anyhow::Result;

use crate::database::pager::HeaderPage;

#[derive(Debug)]
pub struct Database {
    pub db_path: PathBuf,
    pub header_page: HeaderPage
}

impl Database {
    pub fn create(db_path: &str) -> Result<Database> {
        let formatted_db_path = format!("{}.mini", db_path);
        let path = Path::new(&formatted_db_path);

        let mut f = File::create(path)?;

        let header_page = HeaderPage::new();
        header_page.write(&mut f)?;

        Ok(Database {
            db_path: path.to_owned(),
            header_page
        })
    }

    pub fn open(db_path: &str) -> Result<Database> {
        let formatted_db_path = format!("{}.mini", db_path);
        let path = Path::new(&formatted_db_path);
        let mut f = OpenOptions::new().read(true).write(true).open(path)?;
        let header_page = HeaderPage::read_from(&mut f)?;
        println!("{:?}", header_page);

        Ok(Database {
            db_path: path.to_owned(),
            header_page
        })
    }
}
