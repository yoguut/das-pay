mod cli;
mod models;

use crate::cli::Opt;
use crate::models::{Account, Transaction};
use anyhow::{Context, Result};
use csv;
use std::{
    env::{self},
    error::Error,
    io,
};

fn main() -> Result<(), anyhow::Error> {
    let file_path = env::args().nth(1).context("Invalid command")?;
    let opt = Opt::new(file_path).context("Cannot find input file")?;
    let mut rdr = csv::Reader::from_path(opt.path)?;
    let mut wtr = csv::Writer::from_writer(io::stdout());
    let mut results = rdr.deserialize();
    let mut summary: Vec<Transaction> = vec![];

    for (idx, result) in results.enumerate() {
        // coercing `DeserializeRecordsIter` hurts performance
        let err_msg = format!("Invalid record at line {}", idx);
        let trans: Transaction = result.context(err_msg)?;
        // eprintln!("{:?}", &trans);
        summary.push(trans);
    }

    wtr.flush();
    Ok(())
}
