use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about=None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// List balances.
    List,
    /// Add transaction.
    Add(AddArgs),
}

#[derive(Args)]
pub struct ListArgs {}

#[derive(Args)]
pub struct AddArgs {
    pub from: String,
    pub to: String,
    pub value: u64,
}
