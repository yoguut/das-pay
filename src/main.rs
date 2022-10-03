mod opt;

use anyhow::{Context, Result};
use opt::Opt;
use std::env::{self};

fn main() -> Result<(), anyhow::Error> {
    let file_path = env::args().nth(1).context("Invalid command")?;
    let opt = Opt::new(file_path).context("Cannot find input file")?;
    Ok(())
}
