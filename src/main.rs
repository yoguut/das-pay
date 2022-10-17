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
    let accounts = sequential_serde(rdr)?;
    flush(wtr, accounts)
}

/// loop through each transaction for validation
/// complexity: O(n)
fn sequential_serde(mut rdr: csv::Reader<File>) -> Result<Vec<Account>, anyhow::Error> {
    let mut transaction_map = TransactionHM::new();
    let mut account_map = AccountHM::new();

    for (idx, result) in rdr.deserialize().enumerate() {
        let err_msg = format!("Invalid transaction: Malformed object at {}", idx);
        let trans: Transaction = result.context(err_msg)?;

        match trans.get_trans_type().as_str() {
            "deposit" => match account_map.get_mut(&trans.get_client_id()) {
                Some(existing_acc) => {
                    if !existing_acc.locked {
                        let available =
                            existing_acc.get_available() + trans.get_amount().unwrap_or(0_f32);
                        existing_acc.set_available(available);
                    }
                }
                None => {
                    let new_acc = Account::new(
                        trans.get_client_id(),
                        0_f32,
                        trans.get_amount().unwrap_or(0_f32),
                        false,
                    );
                    account_map.insert(trans.get_client_id(), new_acc);
                    transaction_map.insert(trans.get_tx_id(), trans);
                }
            },
            "withdrawal" => {
                let existing_acc = account_map
                    .get_mut(&trans.get_client_id())
                    .context("Fail to withdrawal: Account not found")?;
                // check if account has sufficient available funds
                let withdraw_amount = trans.get_amount().unwrap_or(0_f32);
                if existing_acc.get_available() >= withdraw_amount && !existing_acc.locked {
                    let available = existing_acc.get_available() - withdraw_amount;
                    existing_acc.set_available(available);
                    transaction_map.insert(trans.get_tx_id(), trans);
                }
            }
            "dispute" => {
                let existing_acc = account_map
                    .get_mut(&trans.get_client_id())
                    .context("Fail to dispute: Account not found")?;
                if !existing_acc.locked {
                    if let Some(existing_trans) = transaction_map.get_mut(&trans.get_tx_id()) {
                        let amount = existing_trans.get_amount().unwrap_or(0_f32);
                        existing_acc.set_held(existing_acc.get_held() + amount);
                        existing_acc.set_available(existing_acc.get_available() - amount);
                    }
                }
            }
            "resolve" => {
                let existing_acc = account_map
                    .get_mut(&trans.get_client_id())
                    .context("Fail to resolve: Account not found")?;
                if !existing_acc.locked {
                    if let Some(existing_trans) = transaction_map.get_mut(&trans.get_tx_id()) {
                        let amount = existing_trans.get_amount().unwrap_or(0_f32);
                        existing_acc.set_available(existing_acc.get_available() + amount);
                        existing_acc.set_held(existing_acc.get_held() - amount);
                    }
                }
            }
            "chargeback" => {
                let existing_acc = account_map
                    .get_mut(&trans.get_client_id())
                    .context("Fail to chargeback: Account not found")?;
                if !existing_acc.locked {
                    if let Some(existing_trans) = transaction_map.get_mut(&trans.get_tx_id()) {
                        let amount = existing_trans.get_amount().unwrap_or(0_f32);
                        existing_acc.set_available(existing_acc.get_available() - amount);
                        existing_acc.set_held(existing_acc.get_held() - amount);
                        existing_acc.locked = true;
                    }
                }
            }
            _ => continue,
        }
    }
    Ok(account_map.values().cloned().collect::<Vec<Account>>())
}

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
