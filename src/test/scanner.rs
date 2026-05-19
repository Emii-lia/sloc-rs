use std::fs::create_dir_all;
use crate::scanner::scan_file;
use crate::test::{ProjectBuf, CWD_LOCK};

#[test]
fn scanner_counts_code_and_blank_lines() {
  let _guard = CWD_LOCK.lock().unwrap_or_else(|e| e.into_inner());
  let project = ProjectBuf::new("scanner_counts_code_and_blank_lines");
  let original_cwd = std::env::current_dir().unwrap();
  create_dir_all(&project.path).unwrap();
  project.create_file_with_content("file.txt", "Line 1\n\nLine 2\n\nLine 3");
  std::env::set_current_dir(&project.path).unwrap();
  let stats = scan_file(project.path.join("file.txt"));
  assert_eq!(stats.code, 3);
  assert_eq!(stats.blank, 2);
  std::env::set_current_dir(original_cwd).unwrap();
}