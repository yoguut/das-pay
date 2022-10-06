mod cli;
mod models;

use crate::cli::Opt;
use crate::models::{Account, Transaction};
use anyhow::{Context, Result};
use csv;
use std::{
    collections::HashMap,
    env::{self},
    error::Error,
    io,
};

fn main() -> Result<(), anyhow::Error> {
    let file_path = env::args().nth(1).context("Invalid command")?;
    let opt = Opt::new(file_path).context("Cannot find input file")?;
    let mut rdr = csv::Reader::from_path(opt.path)?;
    let mut wtr = csv::Writer::from_writer(io::stdout());
    let mut map: HashMap<u16, Account> = HashMap::new();

    // O(n)
    for (idx, result) in rdr.deserialize().enumerate() {
        // coercing `DeserializeRecordsIter` hurts performance
        let err_msg = format!("Invalid record at line {}", idx);
        let trans: Transaction = result.context(err_msg)?;
        match map.get(&trans.client_id) {
            Some(id) => {
                // process & update map
                println!("existing id {} in map", id);
            },
            None => {
                let deposit = String::from("deposit");
                match trans.trans_type {
                    deposit => {
                        // valid operation
                        let acc = Account::new(trans.client_id, 0.0_f32, trans.amount, false);
                        map.insert(trans.client_id, acc);
                    },
                    _ => continue,
                }
            },
        }
        // eprintln!("{:?}", &trans);
    }

    eprintln!("{:?}", map);
    wtr.flush();
    Ok(())
}
