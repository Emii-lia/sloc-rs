use std::path::PathBuf;
use crate::types::Stats;

pub fn scan_file(path: PathBuf) -> Stats {
  let content = std::fs::read(path).unwrap_or_default();

  let mut code = 0;
  let mut blank = 0;

  for line in content.split(|&b| b == b'\n') {
    let trimmed = line.trim_ascii();

    if trimmed.is_empty() {
      blank += 1;
    } else {
      code += 1;
    }
  }

  Stats { code, blank }
}