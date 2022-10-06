mod cli;
mod models;

use crate::cli::Opt;
use crate::models::{Account, Transaction};
use anyhow::{Context, Result};
use csv;
use std::env::{self};
use std::error::Error;

fn main() -> Result<(), anyhow::Error> {
    let file_path = env::args().nth(1).context("Invalid command")?;
    let opt = Opt::new(file_path).context("Cannot find input file")?;
    let mut rdr = csv::Reader::from_path(opt.path)?;
    let mut results = rdr.deserialize();
    let mut summary: Vec<Transaction> = vec![];

    for result in results {
        // coercing `DeserializeRecordsIter` hurts performance
        let trans: Transaction = result?;
        summary.push(trans);
        // eprintln!("{:?}", trans);
    }

    let mut wtr = csv::Writer::from_writer(summary);
    wtr.serialize
    Ok(())
}

fn audit_tansactions(transactions: &mut DeserializeRecordsIter) -> Result<(), Box<dyn Error>> {
    for x in transactions {
        let trans: Transaction = x?;
        eprintln!("{:?}", trans);
    }
    Ok(())
}
