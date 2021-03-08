use std::{sync::{Arc, Mutex}, thread, time};
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

use rand::{Rng, thread_rng};

use crate::logger::Logger;


pub struct Provider {
    current_hash: Mutex<u64>,
    logger: Arc<Logger>,
    finish: Mutex<bool>
}

impl Provider {
    pub fn new(logger: Arc<Logger>) -> Provider{
        Provider{
            current_hash: Mutex::new(Provider::generate_hash()),
            logger,
            finish: Mutex::new(false),
        }
    }

    pub fn start(&self) {
        while !*self.finish.lock().unwrap() {
            thread::sleep(time::Duration::from_millis(1000));
            let mut value = self.current_hash.lock().unwrap();
            *value = Provider::generate_hash();
        }
    }

    pub fn stop(&self) {
        let mut finish = self.finish.lock().unwrap();
        *finish = true;
    }

    pub fn get_hash(&self) -> u64 {
        let mut value = self.current_hash.lock().unwrap();
        let aux_value = *value;
        *value = Provider::generate_hash();
        aux_value
    }

    fn generate_hash() -> u64 {
        let mut rng = thread_rng();
        let number: u32 = rng.gen_range(1..45988);
        Provider::calculate_hash(number)
    }

    pub fn calculate_hash(t: u32) -> u64 {
        let mut s = DefaultHasher::new();
        t.hash(&mut s);
        s.finish()
    }
}