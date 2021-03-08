mod cash_operation_type;
mod cash_worker;
mod data_faker;
mod logger;
mod processing_worker;
mod transaction;

use cash_operation_type::CashOperationType;
use cash_worker::CashWorker;
use logger::Logger;
use processing_worker::ProcessingWorker;
use std::env;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::sync::mpsc::{sync_channel, Receiver, SyncSender};
use std::sync::{Arc, Mutex};
use std::thread;
use transaction::Transaction;
use uuid::Uuid;

const ARGS_LEN: u8 = 5;
const ARGS_LEN_WITH_DEBUG: u8 = 6;
const FILE_PATH: &str = "src/transactions.csv";

fn load_transactions(path: &str) -> Vec<Transaction> {
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

fn parse_args() -> (u8, u8, u8, u8, bool) {
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

fn run_cash_worker(
    logger: Arc<Logger>,
    worker_type: CashOperationType,
    rx: Receiver<Transaction>,
) -> thread::JoinHandle<()> {
    let mut worker = CashWorker::new(logger, worker_type, rx);
    thread::spawn(move || {
        worker.start();
    })
}

fn run_processing_workers(
    cashin_txs: Vec<SyncSender<Transaction>>,
    cashout_txs: Vec<SyncSender<Transaction>>,
    transactions: Arc<Mutex<Vec<Transaction>>>,
    logger: Arc<Logger>,
) -> thread::JoinHandle<()> {
    let mut worker = ProcessingWorker::new(transactions, cashin_txs, cashout_txs, logger);
    thread::spawn(move || {
        worker.start();
    })
}

fn main() {
    let (ia_workers_amount, cashin_workers_amount, cashout_workers_amount, clients_amount, debug) =
        parse_args();

    if ia_workers_amount == 0 {
        println!("USAGE:\n[IA_WORKERS] [CASHIN_WORKERS] [CASHOUT_WORKERS] [CLIENTS]");
        return;
    }
    let logger = Arc::new(Logger::init_logger("log", debug));

    let mut user_ids: Vec<String> = Vec::new();
    for _i in 0..clients_amount {
        user_ids.push(Uuid::new_v4().to_string());
    }
    data_faker::generate_data(FILE_PATH, &user_ids);

    let mut cash_in_txs: Vec<SyncSender<Transaction>> = Vec::new();
    let mut cash_out_txs: Vec<SyncSender<Transaction>> = Vec::new();

    let mut cash_in_workers: Vec<thread::JoinHandle<()>> = Vec::new();
    for _i in 0..cashin_workers_amount {
        let (tx, rx) = sync_channel::<Transaction>(1);
        cash_in_txs.push(tx);
        cash_in_workers.push(run_cash_worker(
            Arc::clone(&logger),
            CashOperationType::CashIn,
            rx,
        ))
    }

    let mut cash_out_workers: Vec<thread::JoinHandle<()>> = Vec::new();
    for _i in 0..cashout_workers_amount {
        let (tx, rx) = sync_channel::<Transaction>(1);
        cash_out_txs.push(tx);
        cash_out_workers.push(run_cash_worker(
            Arc::clone(&logger),
            CashOperationType::CashOut,
            rx,
        ))
    }

    let transactions = load_transactions(FILE_PATH);
    let transactions_protect = Arc::new(Mutex::new(transactions));

    let mut ia_workers: Vec<thread::JoinHandle<()>> = Vec::new();
    for _i in 0..ia_workers_amount {
        let cashin_txs_temp = cash_in_txs.iter().map(|x| x.clone()).collect();
        let cashout_txs_temp = cash_out_txs.iter().map(|x| x.clone()).collect();
        ia_workers.push(run_processing_workers(
            cashin_txs_temp,
            cashout_txs_temp,
            Arc::clone(&transactions_protect),
            Arc::clone(&logger),
        ));
    }

    drop(cash_in_txs);
    drop(cash_out_txs);
    for processing_worker in ia_workers {
        processing_worker.join().unwrap();
    }
    for worker in cash_in_workers {
        worker.join().unwrap();
    }
    for worker in cash_out_workers {
        worker.join().unwrap();
    }
}
