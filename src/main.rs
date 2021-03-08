mod cash_operation_type;
mod cash_worker;
mod data_faker;
mod final_worker;
mod helpers;
mod logger;
mod processing_worker;
mod provider;
mod transaction;
mod user;

use cash_operation_type::CashOperationType;
use transaction::Transaction;
use user::User;
use final_worker::FinalWorker;
use helpers::*;
use logger::Logger;
use std::{collections::HashMap, sync::mpsc::sync_channel};
use std::{sync::{Arc, Mutex}, thread};
use uuid::Uuid;

const FILE_PATH: &str = "src/transactions.csv";
const TRANSACTION_BUFFER_SIZE: usize = 256;

fn main() {
    env_logger::init();
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

    let mut users: HashMap<String, User> = HashMap::new();
    for id in user_ids {
        users.insert(id.clone(), User::new(id));
    }
    
    let (final_worker_tx, final_worker_rx) = sync_channel::<Transaction>(TRANSACTION_BUFFER_SIZE);
    let final_worker_handle = get_final_worker(Arc::clone(&logger), final_worker_rx, users);

    let hash_generator = Arc::new(get_hash_generator(Arc::clone(&logger)));
    let hash_gen_arc = Arc::clone(&hash_generator);
    let hash_generator_th = thread::spawn(move || {
        hash_gen_arc.start();
    });

    let (cash_in_txs, cash_in_workers) = get_cash_workers(
        cashin_workers_amount,
        CashOperationType::CashIn,
        Arc::clone(&logger),
        Arc::clone(&hash_generator),
        final_worker_tx.clone()
    );
    let (cash_out_txs, cash_out_workers) = get_cash_workers(
        cashout_workers_amount,
        CashOperationType::CashOut,
        Arc::clone(&logger),
        Arc::clone(&hash_generator),
        final_worker_tx
    );

    let transactions = load_transactions(FILE_PATH);
    let transactions_protect = Arc::new(Mutex::new(transactions));
    let ia_workers = get_ia_workers(
        ia_workers_amount,
        transactions_protect,
        Arc::clone(&logger),
        cash_in_txs,
        cash_out_txs,
    );

    for processing_worker in ia_workers {
        processing_worker.join().unwrap();
    }
    for worker in cash_in_workers {
        worker.join().unwrap();
    }
    for worker in cash_out_workers {
        worker.join().unwrap();
    }
    hash_generator.stop();
    hash_generator_th.join().unwrap();
    final_worker_handle.join().unwrap();
}
