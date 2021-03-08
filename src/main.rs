mod cash_operation_type;
mod cash_worker;
mod data_faker;
mod helpers;
mod logger;
mod processing_worker;
mod provider;
mod transaction;

use cash_operation_type::CashOperationType;
use helpers::{parse_args, get_cash_workers, get_ia_workers, load_transactions};
use logger::Logger;
use std::sync::{Arc, Mutex};
use uuid::Uuid;

const FILE_PATH: &str = "src/transactions.csv";

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

    let (cash_in_txs, cash_in_workers) = get_cash_workers(
        cashin_workers_amount,
        CashOperationType::CashIn,
        Arc::clone(&logger),
    );

    let (cash_out_txs, cash_out_workers) = get_cash_workers(
        cashout_workers_amount,
        CashOperationType::CashOut,
        Arc::clone(&logger),
    );

    let transactions = load_transactions(FILE_PATH);
    let transactions_protect = Arc::new(Mutex::new(transactions));

    let ia_workers = get_ia_workers(ia_workers_amount, transactions_protect, logger, cash_in_txs, cash_out_txs);

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
