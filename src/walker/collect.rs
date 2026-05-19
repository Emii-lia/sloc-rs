use std::path::PathBuf;
use crate::utils::ignore::{IGNORE_DIR, IGNORE_FILE};
use crate::utils::support::is_supported_file_extension;

pub fn collect_files(root: PathBuf) -> Vec<PathBuf> {
  let mut files = vec![];
  let mut stack = vec![root];
  while let Some(dir) = stack.pop() {
    if dir.is_file() {
      if !IGNORE_FILE.is_ignored(&dir.to_str().unwrap())
        && is_supported_file_extension(&dir.to_str().unwrap())
      {
        files.push(dir);
      }
      continue;
    }
    let Ok(entries) = std::fs::read_dir(dir) else {
      continue;
    };
    for entry in entries.flatten() {
      let path = entry.path();
      if path.is_dir() {
        if !IGNORE_DIR.is_ignored(&path.file_name().unwrap().to_str().unwrap()) {
          stack.push(path);
        }
      } else {
        if
          !IGNORE_FILE.is_ignored(&path.file_name().unwrap().to_str().unwrap())
          && is_supported_file_extension(&path.file_name().unwrap().to_str().unwrap())
        {
        files.push(path);
        }
      }
    }
  }
      files
}