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
    for result in rdr.deserialize() {
        let trans: Transaction = result?;
        eprintln!("{:?}", trans);
    }
    Ok(())
}
