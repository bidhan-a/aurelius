mod cli;
mod store;

use anyhow::Result;
use clap::Parser;
use cli::{Cli, Commands};
use std::path::Path;
use store::{State, Transaction};

fn main() -> Result<()> {
    let cli = Cli::parse();

    let mut state = State::new(Path::new("tx.db"))?;

    match &cli.command {
        Commands::List => {
            println!("{}", state);
        }
        Commands::Add(args) => {
            let tx = Transaction::new(String::from(&args.from), String::from(&args.to), args.value);
            state.add(tx)?;
            state.save()?;
        }
    }

    Ok(())
}
