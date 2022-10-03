mod opt;

use anyhow::{Result};
use opt::Opt;
use std::env::{self};

fn main() -> Result<(), anyhow::Error> {
    let file_path = env::args().nth(1);
    let opt = Opt::new(file_path).ok_or_else("Cannot find input file")?;
    println!("{:?}", opt);
    Ok(())
}
