use std::sync::mpsc;
use logger::Logger;
use user::User;

pub struct FinalWorker {
    users: Vec<User>,
    logger: Arc<Logger>,
    channel: mpsc::Receiver<Transaction>
}

impl FinalWorker {
    pub fn new(user_ids: Vec<String>, logger: Arc<Logger>) {
        let mut users: Vec<User> = Vec::new()
        for id in user_ids {
            users.push(User::new(id))
        }
        FinalWorker {
            users,
            logger
        }
    }

    pub fn start(&mut self) {
        for transaction in self.channel {
            let iter = self.channel.iter();
            let user = iter.find(|usr| usr.id == transaction.user_id);
            if transaction.operation_type == CashOperationType::CashIn {
                user.update_balance(transaction.amount)
            }
            else {
                user.update_balance(-1 * transaction.amount)
            }
        }

        for user in self.users {
            println!("User {}:\t{}", user.id, user.balance)
        }
    }
}