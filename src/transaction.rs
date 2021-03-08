use crate::cash_operation_type::CashOperationType;

#[derive(Debug, Clone)]
pub struct Transaction {
    id: u8,
    user_id: String,
    timestamp: u32,
    operation_type: CashOperationType,
    amount_of_cash: f64,
    authentication_hash: u64,
}

impl Transaction {
    pub fn new(
        id: u8,
        user_id: String,
        timestamp: u32,
        operation_type: CashOperationType,
        amount_of_cash: f64,
    ) -> Transaction {
        Transaction {
            id,
            user_id,
            timestamp,
            operation_type,
            amount_of_cash,
            authentication_hash: 0,
        }
    }

    pub fn get_transaction_type(&self) -> &CashOperationType {
        &self.operation_type
    }

    pub fn set_authentication_hash(&mut self, hash: u64) {
        self.authentication_hash = hash;
    }
}
