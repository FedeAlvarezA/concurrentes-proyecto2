use crate::logger::Logger;
use crate::processing_worker::ProcessingWorker;
use crate::transaction::Transaction;
use crate::FinalWorker;
use crate::{cash_operation_type::CashOperationType, provider::Provider};
use crate::{cash_worker::CashWorker, user::User};
use std::io::{prelude::*, BufReader};
use std::sync::mpsc::{sync_channel, Receiver, SyncSender};
use std::sync::{Arc, Mutex};
use std::thread;
use std::{collections::HashMap, fs::File};
use std::{env, thread::JoinHandle};

const ARGS_LEN: u8 = 5;
const ARGS_LEN_WITH_DEBUG: u8 = 6;

pub fn load_transactions(path: &str) -> Vec<Transaction> {
    let mut transactions: Vec<Transaction> = Vec::new();

    let file = File::open(path).expect("Something went wrong reading the file");
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let elements: Vec<String> = line.unwrap().split(',').map(str::to_string).collect();
        let transaction_type = match &elements[3][..] {
            "cash_in" => CashOperationType::CashIn,
            _ => CashOperationType::CashOut,
        };
        let transaction = Transaction::new(
            elements[0].parse::<u8>().unwrap(),
            elements[1].clone(),
            elements[2].parse::<u32>().unwrap(),
            transaction_type,
            elements[4].parse::<f64>().unwrap(),
        );
        transactions.push(transaction);
    }
    transactions
}

pub fn parse_args() -> (u8, u8, u8, u8, bool) {
    let args: Vec<String> = env::args().collect();
    let mut debug = false;

    match args.len() as u8 {
        ARGS_LEN => {}
        ARGS_LEN_WITH_DEBUG => debug = true,
        _ => return (0, 0, 0, 0, false),
    }
    (
        args[1].parse().unwrap(),
        args[2].parse().unwrap(),
        args[3].parse().unwrap(),
        args[4].parse().unwrap(),
        debug,
    )
}

pub fn run_cash_worker(
    id: u8,
    logger: Arc<Logger>,
    worker_type: CashOperationType,
    rx: Receiver<Transaction>,
    hash_generator: Arc<Provider>,
    final_worker_tx: SyncSender<Transaction>,
) -> thread::JoinHandle<()> {
    let worker = CashWorker::new(id, logger, worker_type, rx, hash_generator, final_worker_tx);
    thread::spawn(move || {
        worker.start();
    })
}

pub fn run_processing_workers(
    id: u8, 
    cashin_txs: Vec<SyncSender<Transaction>>,
    cashout_txs: Vec<SyncSender<Transaction>>,
    transactions: Arc<Mutex<Vec<Transaction>>>,
    logger: Arc<Logger>,
) -> thread::JoinHandle<()> {
    let mut worker = ProcessingWorker::new(id, transactions, cashin_txs, cashout_txs, logger);
    thread::spawn(move || {
        worker.start();
    })
}

pub fn get_cash_workers(
    cash_workers_amount: u8,
    cash_op_type: CashOperationType,
    logger: Arc<Logger>,
    hash_generator: Arc<Provider>,
    final_worker_tx: SyncSender<Transaction>,
) -> (Vec<SyncSender<Transaction>>, Vec<JoinHandle<()>>) {
    let mut cash_txs: Vec<SyncSender<Transaction>> = Vec::new();
    let mut cash_workers: Vec<thread::JoinHandle<()>> = Vec::new();

    for _i in 0..cash_workers_amount {
        let (tx, rx) = sync_channel::<Transaction>(1);
        cash_txs.push(tx);
        cash_workers.push(run_cash_worker(
            _i,
            Arc::clone(&logger),
            cash_op_type,
            rx,
            Arc::clone(&hash_generator),
            final_worker_tx.clone(),
        ))
    }
    (cash_txs, cash_workers)
}

pub fn get_ia_workers(
    ia_workers_amount: u8,
    transactions_protect: Arc<Mutex<Vec<Transaction>>>,
    logger: Arc<Logger>,
    cash_in_txs: Vec<SyncSender<Transaction>>,
    cash_out_txs: Vec<SyncSender<Transaction>>,
) -> Vec<JoinHandle<()>> {
    let mut ia_workers: Vec<thread::JoinHandle<()>> = Vec::new();
    for _i in 0..ia_workers_amount {
        let cashin_txs_temp = cash_in_txs.iter().map(|x| x.clone()).collect();
        let cashout_txs_temp = cash_out_txs.iter().map(|x| x.clone()).collect();
        ia_workers.push(run_processing_workers(
            _i,
            cashin_txs_temp,
            cashout_txs_temp,
            Arc::clone(&transactions_protect),
            Arc::clone(&logger),
        ));
    }
    ia_workers
}

pub fn get_hash_generator(logger: Arc<Logger>) -> Provider {
    Provider::new(logger)
}

pub fn get_final_worker(
    logger: Arc<Logger>,
    rx: Receiver<Transaction>,
    users: HashMap<String, User>,
) -> JoinHandle<()> {
    let mut final_worker = FinalWorker::new(users, logger, rx);
    thread::spawn(move || {
        final_worker.start();
    })
}
