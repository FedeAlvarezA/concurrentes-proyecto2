use std::sync::mpsc;
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;


pub struct Provider {
    current_hash: Mutex<u64>,
    logger: Arc<Logger>,
    channel: mpsc::Sender<String>,
    finish: Arc<Mutex<bool>>
}

impl Provider {
    pub fn new(logger: Arc<Logger>, channel: mpsc::Sender<String>){
        Provider{
            0,
            self.calculate_hash(0)
            logger,
            channel
        }
    }

    pub fn start(&mut self) {
        while !*finish.lock().unwrap() {
            thread::sleep(time::Duration::from_millis(1000))
            let value = self.current_hash.lock().unwrap();
            *value = self.generate_hash();
        }
    }

    pub fn get_hash(&mut self) -> u64 {
        let value = self.current_hash.lock().unwrap();
        *value
    }

    fn generate_hash(&mut self){
        let mut rng = rand::thread_rng();
        let number: u32 = rng.gen_range(45988);
        calculate_hash(number)
    }

    fn calculate_hash(t: u32) -> u64 {
        let mut s = DefaultHasher::new();
        t.hash(&mut s);
        s.finish()
    }
}