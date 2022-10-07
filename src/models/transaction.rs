use serde::Deserialize;
use std::fmt;

/// type,client,tx,amount
/// deposit,1,1,1
/// deposit,2,2,2
/// deposit,1,3,2
/// withdrawal,1,4,1.5
/// withdrawal,2,5,3
#[derive(Debug, Deserialize)]
pub struct Transaction {
    #[serde(rename(deserialize = "type"))]
    pub trans_type: String,
    #[serde(rename(deserialize = "client"))]
    pub client_id: u16,
    #[serde(rename(deserialize = "tx"))]
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
        write!(
            f,
            "trans_type/type: {}, client_id/client: {}, tx_id/tx: {}, amount: {}",
            self.trans_type, self.client_id, self.tx_id, self.amount
        )
    }
}
