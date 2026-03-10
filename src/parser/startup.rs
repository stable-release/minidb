use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "minidb", version, about = "MiniDB CLI")]
pub struct StartupCli {
    #[arg(name = "DB_NAME")]
    pub db_name: Option<String>,

    #[arg(short = 'n', long)]
    pub new: bool,
}