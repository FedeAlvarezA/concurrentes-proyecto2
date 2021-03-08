use std::{collections::HashMap, sync::{Arc, mpsc::Receiver}};
use crate::{cash_operation_type::CashOperationType, logger::Logger, transaction::Transaction};
use crate::user::User;

pub struct FinalWorker {
    users: HashMap<String, User>,
    logger: Arc<Logger>,
    rx: Receiver<Transaction>
}

impl FinalWorker {
    pub fn new(users: HashMap<String, User>, logger: Arc<Logger>, rx: Receiver<Transaction>) -> FinalWorker {
        FinalWorker {
            users,
            logger,
            rx
        }
    }

    pub fn start(&mut self) {
        loop {
            let transaction_status = self.rx.recv();
            match transaction_status {
                Err(_) => break,
                _ => {}
            };
            let transaction = transaction_status.unwrap();
            let mut transaction_amount = transaction.get_transaction_amount();
            let user_id = transaction.get_user_id();
            let user = self.users.get_mut(user_id).unwrap();
            self.logger.log(format!("Final worker:\treceived {:?}", transaction));
            match transaction.get_transaction_type() {
                CashOperationType::CashOut => {transaction_amount *= -1.0},
                _ => {}
            }
            user.update_balance(transaction_amount);
        };
        self.logger.log(format!("Final balances"));
        for (_id, user) in self.users.iter() {
            self.logger.log(format!("{:?}", user));
        }
    }
}