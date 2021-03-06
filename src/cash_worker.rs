use std::sync::Arc;
use crate::logger::Logger;
use crate::worker_types::CashWorkerTypes;

pub struct CashWorker {
    id: u8,
    logger: Arc<Logger>,
    worker_type: CashWorkerTypes
}

impl CashWorker {
    pub fn new(id: u8, logger: Arc<Logger>, worker_type: CashWorkerTypes) -> CashWorker {
        CashWorker {
            id,
            logger,
            worker_type
        }
    }

    pub fn start(&mut self) {

    }
}