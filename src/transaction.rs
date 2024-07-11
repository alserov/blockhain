use serde_derive::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Transaction {
    pub sender: String,
    pub receiver: String,
    pub amount: f32,
}

impl Transaction {
    pub fn new(sender: String, receiver: String, amount: f32) -> Self {
        Transaction{
            sender,
            receiver,
            amount,
        }
    }
}