mod data_faker;
mod logger;
mod cash_worker;
mod worker_types;

use logger::Logger;
use std::env;
use std::sync::Arc;
use std::{thread};
use uuid::Uuid;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use worker_types::CashWorkerTypes;
use cash_worker::CashWorker;

const ARGS_LEN: u8 = 5;
const ARGS_LEN_WITH_DEBUG: u8 = 6;

fn run_cash_worker(id: u8, logger: Arc<Logger>, worker_type: CashWorkerTypes) -> thread::JoinHandle<()> {
  let mut worker = CashWorker::new(id, logger, worker_type);
  let thr = thread::spawn(move || {
    worker.start();
  });

  return thr;
}

fn main() {
  let (ia_workers_amount, cashin_workers_amount, cashout_workers_amount, clients_amount, debug) = parse_args();

  if ia_workers_amount == 0{
    println!("USAGE:\n[IA_WORKERS] [CASHIN_WORKERS] [CASHOUT_WORKERS] [CLIENTS]");
    return
  }
  let logger = Arc::new(Logger::init_logger("log", debug));

  let mut user_ids: Vec<String> = Vec::new();
  for _i in 0..clients_amount {
    user_ids.push(Uuid::new_v4().to_string());
  }
  data_faker::generate_data("transactions.csv", &user_ids);

  let mut transactions: Vec<(String, String, String, String, String)> = Vec::new();

  let file = File::open("transactions.csv").expect("Something went wrong reading the file");
  let reader = BufReader::new(file);
  for line in reader.lines() {
    let elements: Vec<String> = line.unwrap().split(',').map(str::to_string).collect();
    transactions.push((elements[0].to_string(), elements[1].to_string(), elements[2].to_string(), elements[3].clone(), elements[4].to_string()))
  }

  for trans in transactions {
      println!("{}\t{}\t{}\t{}\t{}", trans.0, trans.1, trans.2, trans.3, trans.4)
  }

  let logger = Arc::new(Logger::init_logger("log", debug));

  let mut cash_in_workers: Vec<thread::JoinHandle<()>> = Vec::new();
  for i in 0..cashin_workers_amount {
    cash_in_workers.push(run_cash_worker(i, Arc::clone(&logger), CashWorkerTypes::CashIn))
  }

  let mut cash_out_workers: Vec<thread::JoinHandle<()>> = Vec::new();
  for i in 0..cashout_workers_amount {
    cash_out_workers.push(run_cash_worker(i, Arc::clone(&logger), CashWorkerTypes::CashOut))
  }


}

fn parse_args() -> (u8, u8, u8, u8, bool) {
  let args: Vec<String> = env::args().collect();
  let mut debug = false;

  match args.len() as u8 {
    ARGS_LEN => {}
    ARGS_LEN_WITH_DEBUG => {debug=true}
    _ => { 
      return (0, 0, 0, 0, false)
    }
  }
  (args[1].parse().unwrap(), args[2].parse().unwrap(), args[3].parse().unwrap(), args[4].parse().unwrap(), debug)
}

fn load_transactions(path: &str, transactions: Vec<(u8, &str, &str, &str, f64)>) {
  
}