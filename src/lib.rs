mod cli;
mod models;

pub use crate::cli::Opt;
pub use crate::models::{Account, Transaction};
use anyhow::Context;
use csv::Reader;
use std::collections::HashMap;
use std::fs::File;

/// <account.id, account>
type AccountHM = HashMap<u16, Account>;
/// <transaction.tx_id, transaction>
type TransactionHM = HashMap<u32, Transaction>;

/// loop through each transaction to accumulate data onto an array of `Account`.
///
/// complexity: O(n)
///
/// # examples
///
/// ```
/// use std::path::PathBuf;
/// use csv::Reader;
/// use das_pay::sequential_serde;
///
/// let file_path = PathBuf::from("./sample_input.csv");
/// let reader = Reader::from_path(file_path).unwrap();
/// let accounts = sequential_serde(reader);
/// assert_eq!(accounts.unwrap().len(), 2);
/// ```
pub fn sequential_serde(mut rdr: Reader<File>) -> Result<Vec<Account>, anyhow::Error> {
    let mut transaction_map = TransactionHM::new();
    let mut account_map = AccountHM::new();

    for (idx, result) in rdr.deserialize().enumerate() {
        let err_msg = format!("Fail to deserialize: Malformed object at {}", idx);
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
                let err_msg = format!(
                    "Fail to withdrawal: Account {} not found",
                    &trans.get_client_id()
                );
                let existing_acc = account_map
                    .get_mut(&trans.get_client_id())
                    .context(err_msg)?;
                // check if account has sufficient available funds
                let withdraw_amount = trans.get_amount().unwrap_or(0_f32);
                if existing_acc.get_available() >= withdraw_amount && !existing_acc.locked {
                    let available = existing_acc.get_available() - withdraw_amount;
                    existing_acc.set_available(available);
                    transaction_map.insert(trans.get_tx_id(), trans);
                }
            }
            "dispute" => {
                let err_msg = format!(
                    "Fail to dispute: Account {} not found",
                    &trans.get_client_id()
                );
                let existing_acc = account_map
                    .get_mut(&trans.get_client_id())
                    .context(err_msg)?;
                if !existing_acc.locked {
                    if let Some(existing_trans) = transaction_map.get_mut(&trans.get_tx_id()) {
                        let amount = existing_trans.get_amount().unwrap_or(0_f32);
                        existing_acc.set_held(existing_acc.get_held() + amount);
                        existing_acc.set_available(existing_acc.get_available() - amount);
                    }
                }
            }
            "resolve" => {
                let err_msg = format!(
                    "Fail to resolve: Account {} not found",
                    &trans.get_client_id()
                );
                let existing_acc = account_map
                    .get_mut(&trans.get_client_id())
                    .context(err_msg)?;
                if !existing_acc.locked {
                    if let Some(existing_trans) = transaction_map.get_mut(&trans.get_tx_id()) {
                        let amount = existing_trans.get_amount().unwrap_or(0_f32);
                        existing_acc.set_available(existing_acc.get_available() + amount);
                        existing_acc.set_held(existing_acc.get_held() - amount);
                    }
                }
            }
            "chargeback" => {
                let err_msg = format!(
                    "Fail to chargeback: Account {} not found",
                    &trans.get_client_id()
                );
                let existing_acc = account_map
                    .get_mut(&trans.get_client_id())
                    .context(err_msg)?;
                if !existing_acc.locked {
                    if let Some(existing_trans) = transaction_map.get_mut(&trans.get_tx_id()) {
                        let amount = existing_trans.get_amount().unwrap_or(0_f32);
                        existing_acc.set_available(existing_acc.get_available() - amount);
                        existing_acc.set_held(existing_acc.get_held() - amount);
                        existing_acc.locked = true;
                    }
                }
            }
            _ => {
                let err_msg = format!("Fail to deserialize: Invalid type at {}", idx);
                let err = anyhow::Error::msg(err_msg);
                return Err(err);
            }
        }
    }
    Ok(account_map.values().cloned().collect::<Vec<Account>>())
}
