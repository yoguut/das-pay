mod cli;
mod models;

use crate::cli::Opt;
use crate::models::{Account, Transaction};
use anyhow::{Context, Result};
use csv;
use std::{
    collections::HashMap,
    env::{self},
    fs::File,
    io,
};

/// account id: account
type AccountHM = HashMap<u16, Account>;
/// transacton id: transaction
type TransactionHM = HashMap<u32, Transaction>;

fn main() -> Result<(), anyhow::Error> {
    let file_path = env::args().nth(1).context("Invalid cmd")?;
    let opt = Opt::new(file_path).context("Invalid cmd: Input file not found")?;
    let rdr = csv::Reader::from_path(opt.path)?;
    let wtr = csv::Writer::from_writer(io::stdout());
    sequential_serde(rdr, wtr)
}

// O(n^2)
// loop through each transaction for validation
fn sequential_serde(
    mut rdr: csv::Reader<File>,
    mut wtr: csv::Writer<io::Stdout>,
) -> Result<(), anyhow::Error> {
    let mut transaction_map = TransactionHM::new();
    let mut account_map = AccountHM::new();

    for (idx, result) in rdr.deserialize().enumerate() {
        let err_msg = format!("Invalid transaction: Malformed object at {}", idx);
        let trans: Transaction = result.context(err_msg)?;

        match trans.trans_type.as_str() {
            "deposit" => match account_map.get_mut(&trans.client_id) {
                Some(existing_acc) => {
                    if !existing_acc.locked {
                        existing_acc.available += trans.amount.unwrap_or(0_f32);
                    }
                }
                None => {
                    let new_acc =
                        Account::new(trans.client_id, 0_f32, trans.amount.unwrap_or(0_f32), false);
                    account_map.insert(trans.client_id, new_acc);
                    transaction_map.insert(trans.tx_id, trans);
                }
            },
            "withdrawal" => {
                let existing_acc = account_map
                    .get_mut(&trans.client_id)
                    .context("Fail to withdrawal: Account not found")?;
                // check if account has sufficient available funds
                let withdraw_amount = trans.amount.unwrap_or(0_f32);
                if existing_acc.available >= withdraw_amount && !existing_acc.locked {
                    existing_acc.available -= withdraw_amount;
                    transaction_map.insert(trans.tx_id, trans);
                }
            }
            "dispute" => {
                let existing_acc = account_map
                    .get_mut(&trans.client_id)
                    .context("Fail to dispute: Account not found")?;
                if !existing_acc.locked {
                    if let Some(existing_trans) = transaction_map.get_mut(&trans.tx_id) {
                        let amount = existing_trans.amount.unwrap_or(0_f32);
                        existing_acc.held += amount;
                        existing_acc.available -= amount;
                    }
                }
            }
            "resolve" => {
                let existing_acc = account_map
                    .get_mut(&trans.client_id)
                    .context("Fail to resolve: Account not found")?;
                if !existing_acc.locked {
                    if let Some(existing_trans) = transaction_map.get_mut(&trans.tx_id) {
                        let amount = existing_trans.amount.unwrap_or(0_f32);
                        existing_acc.available += amount;
                        existing_acc.held -= amount;
                    }
                }
            }
            "chargeback" => {
                let existing_acc = account_map
                    .get_mut(&trans.client_id)
                    .context("Fail to chargeback: Account not found")?;
                if !existing_acc.locked {
                    if let Some(existing_trans) = transaction_map.get_mut(&trans.tx_id) {
                        let amount = existing_trans.amount.unwrap_or(0_f32);
                        existing_acc.available -= amount;
                        existing_acc.held -= amount;
                        existing_acc.locked = true;
                    }
                }
            }
            _ => continue,
        }
    }
    wtr.flush()?;
    Ok(())
}
