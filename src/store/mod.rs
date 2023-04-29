use anyhow::{anyhow, Result};
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fs::{File, OpenOptions},
    io::{BufRead, BufReader, BufWriter, Write},
    path::Path,
};

lazy_static! {
    static ref GENESIS: HashMap<&'static str, u64> = {
        let mut m = HashMap::new();
        m.insert("GENESIS", 1_000_000_000);
        m
    };
}

pub type Address = String;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Transaction {
    from: Address,
    to: Address,
    value: u64,
}

impl Transaction {
    pub fn new(from: Address, to: Address, value: u64) -> Self {
        Self { from, to, value }
    }
}

pub struct State {
    balances: HashMap<Address, u64>,
    mempool: Vec<Transaction>,
    f: File,
}

impl State {
    pub fn new(path: &Path) -> Result<Self> {
        let mut state = Self::init(path)?;
        state.load()?;
        Ok(state)
    }

    pub fn add(&mut self, tx: Transaction) -> Result<()> {
        self.apply(&tx)?;
        self.mempool.push(tx);
        Ok(())
    }

    pub fn save(&mut self) -> Result<()> {
        let mempool = self.mempool.clone();

        let mut writer = BufWriter::new(&mut self.f);
        for tx in mempool.iter() {
            let serialized = serde_json::to_string(tx)?;
            // Write tx to persistent storage.
            writeln!(writer, "{}", serialized)?;
            // Remove tx from mempool.
            self.mempool.remove(0);
        }

        Ok(())
    }

    fn apply(&mut self, tx: &Transaction) -> Result<()> {
        match self.balances.get(&tx.from) {
            Some(&from_balance) => {
                if tx.value > from_balance {
                    return Err(anyhow!("insufficient balance"));
                }

                // Deduct tx value from sender.
                self.balances
                    .insert(String::from(&tx.from), from_balance - tx.value);

                // Add tx value to recipient.
                match self.balances.get(&tx.to) {
                    Some(&to_balance) => {
                        self.balances
                            .insert(String::from(&tx.to), to_balance + tx.value);
                    }
                    None => {
                        self.balances.insert(String::from(&tx.to), tx.value);
                    }
                }
                Ok(())
            }
            None => Err(anyhow!("insufficient balance")),
        }
    }

    fn init(path: &Path) -> Result<Self> {
        let mut balances: HashMap<Address, u64> = HashMap::new();
        for (&key, &value) in GENESIS.iter() {
            balances.insert(String::from(key), value);
        }

        let mempool = Vec::new();

        let f = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .append(true)
            .open(path)?;

        Ok(State {
            balances,
            mempool,
            f,
        })
    }

    fn load(&mut self) -> Result<()> {
        let f_copy = self.f.try_clone()?;
        let reader = BufReader::new(f_copy);
        for line in reader.lines() {
            if let Ok(l) = line {
                let tx: Transaction = serde_json::from_str(&l)?;
                self.apply(&tx)?;
            }
        }

        Ok(())
    }
}
