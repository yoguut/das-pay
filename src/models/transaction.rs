use serde::Deserialize;
use std::fmt;
use std::string::String;

/// type,client,tx,amount
/// deposit,1,1,1
/// deposit,2,2,2
/// deposit,1,3,2
/// withdrawal,1,4,1.5
/// withdrawal,2,5,3
#[derive(Debug, Deserialize)]
pub struct Transaction {
    /// trans_type: [deposit, withdrawal, dispute, resolve, chargeback]
    #[serde(rename(deserialize = "type"))]
    trans_type: String,
    #[serde(rename(deserialize = "client"))]
    client_id: u16,
    #[serde(rename(deserialize = "tx"))]
    tx_id: u32, // globally unique
    amount: Option<f32>,
}

impl Transaction {
    pub fn get_trans_type(&self) -> String {
        self.trans_type.clone()
    }

    pub fn get_client_id(&self) -> u16 {
        self.client_id
    }

    pub fn get_tx_id(&self) -> u32 {
        self.tx_id
    }

    pub fn get_amount(&self) -> Option<f32> {
        self.amount
    }
}

impl fmt::Display for Transaction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "trans_type/type: {}, client_id/client: {}, tx_id/tx: {}, amount: {:?}",
            self.trans_type, self.client_id, self.tx_id, self.amount
        )
    }
}
