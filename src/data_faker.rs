use csv::Writer;
use std::time::{SystemTime, UNIX_EPOCH};
use rand::Rng;
use rand::seq::SliceRandom;

const AMOUNT_OF_TRANSACTIONS: u32 = 30;
const MAX_MONEY_TRANSFER: u32 = 10000;

pub fn generate_data(filename: &str, user_ids: &Vec<String>) -> () {
  let start = SystemTime::now();
  let since_the_epoch = start
      .duration_since(UNIX_EPOCH)
      .expect("Time went backwards");
  let mut curr_timestamp = since_the_epoch.as_secs() as u32;
  let cash_options = vec!["cash_in", "cash_out"];
  let mut rng = rand::thread_rng();

  let amount_of_transactions = rng.gen_range(0..AMOUNT_OF_TRANSACTIONS);

  let mut wtr = Writer::from_path(filename).unwrap();
  for i in 1..(amount_of_transactions * user_ids.len() as u32 + 1) {
    let random_user_id = user_ids.choose(&mut rng).expect("0 users provided");
    let random_cash_option = cash_options.choose(&mut rng).expect("");
    let random_cash_amount = rng.gen::<f64>() * MAX_MONEY_TRANSFER as f64;

    wtr.write_record(&[
      i.to_string(), random_user_id.to_string(), curr_timestamp.to_string(),
      random_cash_option.to_string(), random_cash_amount.to_string()
    ]).unwrap();

    curr_timestamp += (rng.gen::<f64>() * 30000 as f64) as u32;
  }
  wtr.flush().unwrap();
}