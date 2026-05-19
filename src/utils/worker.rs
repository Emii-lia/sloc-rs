use std::path::PathBuf;
use std::sync::{mpsc, Arc, Mutex};
use crate::scanner::scan_file;
use crate::types::Stats;

pub fn worker(
  rx: Arc<Mutex<mpsc::Receiver<PathBuf>>>,
  tx: mpsc::Sender<Stats>,
) {
  loop {
    let path = {
      let lock = rx.lock().unwrap();
      lock.recv()
    };
    
    match path {
      Ok(path) => {
        let stats = scan_file(path);
        tx.send(stats).unwrap();
      }
      Err(_) => break,
    }
  }
}
