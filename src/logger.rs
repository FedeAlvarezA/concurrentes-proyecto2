use std::fs::File;
use std::io::Write;
use std::sync::{Mutex};
use log;

pub struct Logger {
  file: Mutex<File>,
  debug: bool
}

impl Logger {
  pub fn init_logger(filepath: &str, debug: bool) -> Logger {
    Logger {
      file: Mutex::new(File::create(filepath.to_string()).expect("Unable to create file")),
      debug
    }
  }

  pub fn log(&self, data: String) {
    log::info!("{}", data);
    if self.debug {
      let mut file = self.file.lock().unwrap();
      file.write_all(format!("{}\n", data).as_bytes()).expect("Unable to write data");
    }
    
  }
}