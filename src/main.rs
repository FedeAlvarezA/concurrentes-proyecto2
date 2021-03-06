mod data_faker;
mod logger;

use logger::Logger;
use std::env;
use std::sync::Arc;
use uuid::Uuid;

const ARGS_LEN: u8 = 5;
const ARGS_LEN_WITH_DEBUG: u8 = 6;

fn main() {
  let args: Vec<String> = env::args().collect();
  let mut debug = false;

  match args.len() as u8 {
    ARGS_LEN => {}
    ARGS_LEN_WITH_DEBUG => {debug=true}
    _ => {
      println!("USAGE:\n[IA_WORKERS] [CASHIN_WORKERS] [CASHOUT_WORKERS] [CLIENTS]");
    return
    }
  }
  let ia_workers_amount: u8 = args[1].parse().unwrap();
  let cashin_workers_amount: u8 = args[2].parse().unwrap();
  let cashout_workers_amount: u8 = args[3].parse().unwrap();
  let clients_amount: u8 = args[4].parse().unwrap();

  let logger = Arc::new(Logger::init_logger("log", debug));

  let mut user_ids: Vec<String> = Vec::new();
  for _i in 0..clients_amount {
    user_ids.push(Uuid::new_v4().to_string());
  }
  data_faker::generate_data("transactions.csv", &user_ids);
}
