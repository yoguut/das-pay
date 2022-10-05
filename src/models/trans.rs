pub struct Trans {
    pub trans_type: String,
    pub client_id: u16,
    pub tx_id: u32, // globally unique
    pub amount: f32,
}

impl Trans {
    fn new(trans_type: String, client_id: u16, tx_id: u32, amount: f32) -> Self {
        Trans {
            trans_type,
            client_id,
            tx_id,
            amount,
        }
    }
}
