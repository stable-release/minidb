use anyhow::Result;
use clap::{Parser, Subcommand};

use crate::database::database::Database;

#[derive(Parser, Debug)]
#[command(name = "repl", no_binary_name = true, disable_help_flag = true)]
pub struct REPLCommand {
    #[command(subcommand)]
    cmd: REPLSubCommand
}

#[derive(Subcommand, Debug)]
pub enum REPLSubCommand {

}

pub fn handle_dot_commands (db: &mut Database, line: &str) -> Result<()> {
    Ok(())
}