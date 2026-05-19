use std::fs::create_dir_all;
use crate::commands::sloc;
use crate::test::{ProjectBuf, CWD_LOCK};
use crate::types::Config;

#[test]
fn sloc_counts_sloc_and_files() {
  let _guard = CWD_LOCK.lock().unwrap_or_else(|e| e.into_inner());
  let project = ProjectBuf::new("sloc_counts_sloc_and_files");
  let original_cwd = std::env::current_dir().unwrap();
  create_dir_all(&project.path).unwrap();
  std::env::set_current_dir(&project.path).unwrap();
  project.create_file_with_content("file1.txt", "Line 1\n\nLine 2");
  project.create_file_with_content("file2.txt", "Line 1\nLine 2\nLine 3\n\n");
  project.create_file_with_content("file3.txt", "\n\n\n");
  let (stats, file_len) = sloc(&Config {
    root: project.path.clone(),
    threads: None,
  });

  assert_eq!(stats.code, 5);
  assert_eq!(stats.blank, 7);
  assert_eq!(file_len, 3);
  std::env::set_current_dir(original_cwd).unwrap();
}