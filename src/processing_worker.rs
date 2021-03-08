use rand::{thread_rng, Rng};
use std::time;
use std::{sync::Arc, thread};

use crate::cash_operation_type::CashOperationType;
use crate::logger::Logger;
use crate::transaction::Transaction;
use std::sync::mpsc::{SyncSender, TrySendError};
use std::sync::Mutex;

pub struct ProcessingWorker {
    id: u8,
    transactions: Arc<Mutex<Vec<Transaction>>>,
    cash_in_workers_txs: Vec<SyncSender<Transaction>>,
    cash_out_workers_txs: Vec<SyncSender<Transaction>>,
    logger: Arc<Logger>,
}

impl ProcessingWorker {
    pub fn new(
        id: u8,
        transactions: Arc<Mutex<Vec<Transaction>>>,
        cash_in_workers_txs: Vec<SyncSender<Transaction>>,
        cash_out_workers_txs: Vec<SyncSender<Transaction>>,
        logger: Arc<Logger>,
    ) -> ProcessingWorker {
        ProcessingWorker {
            id,
            transactions,
            cash_in_workers_txs,
            cash_out_workers_txs,
            logger,
        }
    }

    pub fn start(&mut self) {
        self.logger.log(format!("IA Worker {}:\ttarted", self.id));
        loop {
            let mut transactions = self.transactions.lock().unwrap();
            if transactions.is_empty() {
                self.logger.log(format!("IA Worker {}:\texited", self.id));
                break;
            }

            let transaction = transactions.remove(0);
            drop(transactions);
            self.logger.log(format!("IA Worker {}:\tstarted processing {:?}",self.id, transaction));
            let mut rng = thread_rng();
            let sleep_time: u64 = rng.gen_range(10..2000);

            thread::sleep(time::Duration::from_millis(sleep_time));

            let cash_workers = match transaction.get_transaction_type() {
                CashOperationType::CashIn => &self.cash_in_workers_txs,
                CashOperationType::CashOut => &self.cash_out_workers_txs,
            };

            self.logger.log(format!("IA Worker {}:\tfinished processing {:?}",self.id, transaction));
            loop {
                // Itera en circulos hasta que haya un worker disponible
                let mut rng = thread_rng();
                let curr_worker_index: usize = rng.gen_range(0..cash_workers.len());
                let tx = &cash_workers[curr_worker_index];
                match tx.try_send(transaction.clone()) {
                    Err(TrySendError::Disconnected(_)) => {
                        continue;
                    }
                    _ => {
                        self.logger.log(format!("IA Worker {}:\tsending {:?} to cash worker {}", self.id, transaction, curr_worker_index));
                        break
                    },
                }
            }
        }
    }
}
