use std::path::PathBuf;

pub struct Config {
  pub root: PathBuf,
  pub threads: Option<usize>,
}

pub struct Stats {
  pub code: usize,
  pub blank: usize,
}

impl Default for Stats {
  fn default() -> Self {
    Self { code: 0, blank: 0 }
  }
}

impl Stats {
  pub fn merge(&mut self, other: Stats) {
    self.code += other.code;
    self.blank += other.blank;
  }
}