pub struct Transaction {
    id: u8,
    user_id: String,
    timestamp: u32,
    operation_type: CashOperationType,
    amount_of_cash: f64,
    authentication_hash: String,
  }