mod cli;
mod models;

use crate::cli::Opt;
use anyhow::{Context, Result};
use std::env::{self};
use std::fs;

fn main() -> Result<(), anyhow::Error> {
    let file_path = env::args().nth(1).context("Invalid command")?;
    let opt = Opt::new(file_path).context("Cannot find input file")?;
    let contents = fs::read_to_string(opt.path)?;
    println!("{:?}", contents);
    Ok(())
}
