mod store;

use anyhow::Result;
use std::path::Path;
use store::{State, Transaction};

fn main() -> Result<()> {
    // Setup store.
    let mut state = State::new(Path::new("tx.db"))?;

    // Add transactions.
    let tx_1 = Transaction::new(String::from("GENESIS"), String::from("ABC"), 1000);
    let tx_2 = Transaction::new(String::from("ABC"), String::from("DEF"), 500);
    let tx_3 = Transaction::new(String::from("DEF"), String::from("GHI"), 250);
    let tx_4 = Transaction::new(String::from("GHI"), String::from("ABC"), 125);

    state.add(tx_1)?;
    state.add(tx_2)?;
    state.add(tx_3)?;
    state.add(tx_4)?;

    // Save transactions to persistent storage.
    state.save()?;

    Ok(())
}
