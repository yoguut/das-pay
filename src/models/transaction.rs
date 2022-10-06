use serde::Deserialize;
use std::fmt;

#[derive(Debug, Deserialize)]
pub struct Transaction {
    pub trans_type: String,
    pub client_id: u16,
    pub tx_id: u32, // globally unique
    pub amount: f32,
}

impl Transaction {
    fn new(trans_type: String, client_id: u16, tx_id: u32, amount: f32) -> Self {
        Transaction {
            trans_type,
            client_id,
            tx_id,
            amount,
        }
    }
}

impl fmt::Display for Transaction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "trans_type: {}, client_id: {}, tx_id: {}, amount: {}", self.trans_type, self.client_id, self.tx_id, self.amount)
    }
}
