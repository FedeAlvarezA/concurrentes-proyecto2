use std::sync::Arc;
use std::time;

pub struct ProcessingWorker {
    id: usize
    transactions: Arc<Mutex<Vec<u8, String, u64, String, f64>>>,
    cash_in_workers: Arc<Mutex<Vec<CashWorker>>>,
    cash_out_workers: Arc<Mutex<Vec<CashWorker>>>
}

impl ProcessingWorker {
    pub fn new(id: usize, transactions: Arc<Mutex<Vec<u8, String, u64, String, f64>>>, 
        cash_in_workers: Arc<Mutex<Vec<CashWorker>>>, cash_out_workers: Arc<Mutex<Vec<CashWorker>>>) -> ProcessingWorker{
            ProcessingWorker {
                id,
                transactions,
                cash_in_workers,
                cash_out_workers
            }
        }

    pub fn start(&mut self) {
        loop {

        let transactions = self.transactions.lock().unwrap();
        
        if transactions.is_empty() {
            break
        }

        let transaction = transactions.remove(0);

        let mut rng = rand::thread_rng();
        let sleep_time: u8 = rng.gen_range(100);

        thread::sleep(time::Duration::from_millis(sleep_time))

        //TODO: hacer las llamadas
        }

    }
}