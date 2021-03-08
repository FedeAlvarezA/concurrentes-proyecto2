use rand::{thread_rng, Rng};
use std::time;
use std::{sync::Arc, thread};

use crate::cash_operation_type::CashOperationType;
use crate::logger::Logger;
use crate::transaction::Transaction;
use std::sync::mpsc::{SyncSender, TrySendError};
use std::sync::Mutex;

pub struct ProcessingWorker {
    transactions: Arc<Mutex<Vec<Transaction>>>,
    cash_in_workers_txs: Vec<SyncSender<Transaction>>,
    cash_out_workers_txs: Vec<SyncSender<Transaction>>,
    logger: Arc<Logger>,
}

impl ProcessingWorker {
    pub fn new(
        transactions: Arc<Mutex<Vec<Transaction>>>,
        cash_in_workers_txs: Vec<SyncSender<Transaction>>,
        cash_out_workers_txs: Vec<SyncSender<Transaction>>,
        logger: Arc<Logger>,
    ) -> ProcessingWorker {
        ProcessingWorker {
            transactions,
            cash_in_workers_txs,
            cash_out_workers_txs,
            logger,
        }
    }

    pub fn start(&mut self) {
        loop {
            let mut transactions = self.transactions.lock().unwrap();
            if transactions.is_empty() {
                break;
            }

            let transaction = transactions.remove(0);

            let mut rng = thread_rng();
            let sleep_time: u64 = rng.gen_range(10..100);

            thread::sleep(time::Duration::from_millis(sleep_time));

            let cash_workers = match transaction.get_transaction_type() {
                CashOperationType::CashIn => &self.cash_in_workers_txs,
                CashOperationType::CashOut => &self.cash_out_workers_txs,
            };

            let mut curr_worker_index = 0;
            loop {
                // Itera en circulos hasta que haya un worker disponible
                curr_worker_index %= cash_workers.len();
                let tx = &cash_workers[curr_worker_index];
                match tx.try_send(transaction.clone()) {
                    Err(TrySendError::Disconnected(_)) => {
                        curr_worker_index += 1;
                        continue;
                    }
                    _ => break,
                }
            }
        }
    }
}
