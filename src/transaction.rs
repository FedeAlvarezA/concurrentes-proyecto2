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

    pub fn get_user_id(&self) -> &String {
        &self.user_id
    }

    pub fn get_transaction_amount(&self) -> f64 {
        self.amount_of_cash
    }

    pub fn set_authentication_hash(&mut self, hash: u64) {
        self.authentication_hash = hash;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::transaction::Transaction;

    #[test]
    fn new_transaction_sets_properties_correctly() {
        let id = 1;
        let user_id = String::from("test");
        let timestamp = 25;
        let operation_type = CashOperationType::CashIn;
        let amount = 54.545;
        let transaction = Transaction::new(id, String::from("test"), timestamp, operation_type, amount);

        assert_eq!(transaction.id, id);
        assert_eq!(transaction.user_id, user_id);
        assert_eq!(transaction.timestamp, timestamp);
        assert_eq!(transaction.amount_of_cash, amount);
    }

    #[test]
    fn set_authentication_hash_changes_property() {
        let mut transaction = Transaction::new(1, String::from("test"), 2, CashOperationType::CashIn, 22.5);
        assert_eq!(transaction.authentication_hash, 0);

        transaction.set_authentication_hash(22);
        assert_eq!(transaction.authentication_hash, 22)
    }
}
