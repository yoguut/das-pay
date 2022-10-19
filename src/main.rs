use anyhow::{Context, Result};
use das_pay::*;
use std::{
    env::{self},
    io,
};

fn main() -> Result<(), anyhow::Error> {
    let file_path = env::args().nth(1).context("Invalid cmd")?;
    let opt = Opt::new(file_path).context("Invalid cmd: Input file not found")?;
    let rdr = csv::Reader::from_path(opt.path)?;
    let wtr = csv::Writer::from_writer(io::stdout());
    let accounts = sequential_serde(rdr)?;
    flush(wtr, accounts)
}

/// loop through accounts to round & serialize account data and then flush to term.
///
/// complexity: O(n)
fn flush(mut wtr: csv::Writer<io::Stdout>, accounts: Vec<Account>) -> Result<(), anyhow::Error> {
    // convert amount to precise up to 4 decimal places
    // without losing the actual account obj's amount
    // inside of account_map
    for acc in accounts {
        let acc = acc.clone().rounded(4);
        wtr.serialize(acc)?;
    }
    wtr.flush()?;
    Ok(())
}
