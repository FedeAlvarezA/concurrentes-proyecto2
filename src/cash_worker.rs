use crate::{cash_operation_type::CashOperationType, provider::Provider};
use crate::logger::Logger;
use crate::transaction::Transaction;
use std::sync::mpsc::{Receiver, RecvError, SyncSender};
use std::sync::Arc;

pub struct CashWorker {
    id: u8,
    logger: Arc<Logger>,
    worker_type: CashOperationType,
    rx: Receiver<Transaction>,
    hash_generator: Arc<Provider>,
    final_worker_tx: SyncSender<Transaction>,
}

impl CashWorker {
    pub fn new(
        id: u8,
        logger: Arc<Logger>,
        worker_type: CashOperationType,
        rx: Receiver<Transaction>,
        hash_generator: Arc<Provider>,
        final_worker_tx: SyncSender<Transaction>,
    ) -> CashWorker {
        CashWorker {
            id,
            logger,
            worker_type,
            rx,
            hash_generator,
            final_worker_tx
        }
    }

    pub fn start(&self) {
        loop {
            let transaction_status = self.rx.recv();
            match transaction_status {
                Err(RecvError) => break,
                _ => {}
            };
            let mut transaction = transaction_status.unwrap();
            self.logger.log(format!("{:?} worker {}:\treceived {:?}", self.worker_type, self.id, transaction));
            self.logger.log(format!("{:?} worker {}:\trequesting authorization hash for {:?}", self.worker_type, self.id, transaction));
            let hash = self.hash_generator.get_hash();
            transaction.set_authentication_hash(hash);
            self.logger.log(format!("{:?} worker {}:\treceived authorization hash {} for {:?}", self.worker_type, self.id, hash, transaction));
            self.logger.log(format!("{:?} worker {}:\tsending {:?} to final worker", self.worker_type, self.id, transaction));
            self.final_worker_tx.send(transaction).unwrap();
        }
    }
}
