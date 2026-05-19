use std::path::PathBuf;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::thread::available_parallelism;
use crate::types::{Config, Stats};
use crate::utils::worker::worker;
use crate::walker::collect::collect_files;

pub fn sloc(config: &Config) -> (Stats, usize) {
  let config_threads = config.threads
    .unwrap_or(
      available_parallelism().map(|n| n.get()).unwrap_or(2)
    );

  let files = collect_files(config.root.clone());
  let files_len = files.len();

  let (tx, rx) = mpsc::channel::<PathBuf>();
  let (stats_tx, stats_rx) = mpsc::channel::<Stats>();

  let rx = Arc::new(Mutex::new(rx));

  for _ in 0..config_threads {
    let rx = Arc::clone(&rx);
    let stats_tx = stats_tx.clone();

    thread::spawn(move || {
      worker(rx, stats_tx);
    });
  }

  for file in files {
    tx.send(file).unwrap();
  }

  drop(tx);
  drop(stats_tx);

  let mut total = Stats::default();

  for stats in stats_rx {
    total.merge(stats);
  }
  
  (total, files_len)
}