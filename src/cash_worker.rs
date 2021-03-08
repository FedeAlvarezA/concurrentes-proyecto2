use crate::cash_operation_type::CashOperationType;
use crate::logger::Logger;
use crate::transaction::Transaction;
use std::sync::mpsc::{Receiver, RecvError};
use std::sync::Arc;

pub struct CashWorker {
    logger: Arc<Logger>,
    worker_type: CashOperationType,
    rx: Receiver<Transaction>,
}

impl CashWorker {
    pub fn new(
        logger: Arc<Logger>,
        worker_type: CashOperationType,
        rx: Receiver<Transaction>,
    ) -> CashWorker {
        CashWorker {
            logger,
            worker_type,
            rx,
        }
    }

    pub fn start(&mut self) {
        loop {
            let transaction_status = self.rx.recv();
            match transaction_status {
                Err(RecvError) => break,
                _ => {}
            };
            let transaction = transaction_status.unwrap();
            println!("La transaccion: {:?}", transaction);
        }
    }
}
