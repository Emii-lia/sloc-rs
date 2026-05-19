use std::fs::create_dir_all;
use crate::test::{ProjectBuf, CWD_LOCK};
use crate::walker::collect::collect_files;

#[test]
fn collect_files_returns_expected_files() {
  let _guard = CWD_LOCK.lock().unwrap_or_else(|e| e.into_inner());
  let project = ProjectBuf::new("collect_files_returns_expected_files");
  let original_cwd = std::env::current_dir().unwrap();
  create_dir_all(&project.path).unwrap();
  std::env::set_current_dir(&project.path).unwrap();
  project.generate_files(10);

  let files = collect_files(project.path.clone());
  assert_eq!(files.len(), 10);
  std::env::set_current_dir(original_cwd).unwrap();
}

#[test]
fn collect_files_returns_one_on_file() {
  let _guard = CWD_LOCK.lock().unwrap_or_else(|e| e.into_inner());
  let project = ProjectBuf::new("collect_files_returns_one_on_file");
  let original_cwd = std::env::current_dir().unwrap();
  create_dir_all(&project.path).unwrap();
  std::env::set_current_dir(&project.path).unwrap();
  let file = project.create_file("file.txt");

  let files = collect_files(file.clone());
  assert_eq!(files.len(), 1);
  assert_eq!(files[0], file);
  std::env::set_current_dir(original_cwd).unwrap();
}
#[test]
fn collect_files_supports_nested_dir() {
  let _guard = CWD_LOCK.lock().unwrap_or_else(|e| e.into_inner());
  let project = ProjectBuf::new("collect_files_supports_nested_dir");
  let original_cwd = std::env::current_dir().unwrap();
  create_dir_all(&project.path).unwrap();
  std::env::set_current_dir(&project.path).unwrap();
  project.create_dir_with_files("dir1", 5, None);
  project.create_dir_with_files("dir2", 5, None);
  project.create_dir_with_files("dir2/dir2-1", 5, None);
  project.create_dir_with_files("dir2/dir2-2", 5, None);

  let files = collect_files(project.path.clone());
  assert_eq!(files.len(), 20);
  std::env::set_current_dir(original_cwd).unwrap();
}

#[test]
fn collect_files_ignores_hidden_files() {
  let _guard = CWD_LOCK.lock().unwrap_or_else(|e| e.into_inner());
  let project = ProjectBuf::new("collect_files_ignores_hidden_files");
  let original_cwd = std::env::current_dir().unwrap();
  create_dir_all(&project.path).unwrap();
  std::env::set_current_dir(&project.path).unwrap();
  project.create_file(".hidden.txt");
  project.create_file(".hidden2.txt");
  project.create_file("file.txt");

  let files = collect_files(project.path.clone());
  assert_eq!(files.len(), 1);
  assert_eq!(files[0], project.path.join("file.txt"));
  std::env::set_current_dir(original_cwd).unwrap();
}

#[test]
fn collect_files_ignores_hidden_dirs() {
  let _guard = CWD_LOCK.lock().unwrap_or_else(|e| e.into_inner());
  let project = ProjectBuf::new("collect_files_ignores_hidden_dirs");
  let original_cwd = std::env::current_dir().unwrap();
  create_dir_all(&project.path).unwrap();
  std::env::set_current_dir(&project.path).unwrap();
  project.create_dir(".hidden");
  project.create_dir(".hidden2");
  project.create_dir("dir");
  project.create_dir("dir/dir2");
  project.create_file("dir/file.txt");
  project.create_file("dir/dir2/file.txt");
  project.create_file("file.txt");
  project.create_file(".hidden/file.txt");
  project.create_file(".hidden2/file.txt");

  let files = collect_files(project.path.clone());
  assert_eq!(files.len(), 3);
  std::env::set_current_dir(original_cwd).unwrap();
}

#[test]
fn collect_files_ignores_non_project_files() {
  let _guard = CWD_LOCK.lock().unwrap_or_else(|e| e.into_inner());
  let project = ProjectBuf::new("collect_files_ignores_non_project_files");
  let original_cwd = std::env::current_dir().unwrap();
  create_dir_all(&project.path).unwrap();
  std::env::set_current_dir(&project.path).unwrap();
  project.create_file(".env");
  project.create_file("Cargo.lock");
  project.create_file("yarn.lock");
  project.create_file("pnpm-lock.yaml");
  project.create_file("bun.lockb");
  project.create_file("package-lock.json");
  project.create_file("file.txt");
  project.create_file("file2.rs");
  project.create_file("file2");
  project.create_file("file.log");
  project.create_file("file.tmp");
  project.create_file("file.md");
  project.create_file("file.css.map");

  let files = collect_files(project.path.clone());
  assert_eq!(files.len(), 3);
  std::env::set_current_dir(original_cwd).unwrap();
}

#[test]
fn collect_files_ignores_non_project_dirs() {
  let _guard = CWD_LOCK.lock().unwrap_or_else(|e| e.into_inner());
  let project = ProjectBuf::new("collect_files_ignores_non_project_dirs");
  let original_cwd = std::env::current_dir().unwrap();
  create_dir_all(&project.path).unwrap();
  std::env::set_current_dir(&project.path).unwrap();
  project.create_dir_with_files("node_modules", 5, None);
  project.create_dir_with_files("dist", 5, None);
  project.create_dir_with_files("build", 5, None);
  project.create_dir_with_files("vendor", 5, None);
  project.create_dir_with_files("logs", 5, None);
  project.create_dir_with_files("test", 5, None);
  project.create_dir_with_files("tests", 5, None);
  project.create_dir_with_files("public", 5, None);
  project.create_dir_with_files("src", 5, None);
  project.create_file("file.txt");

  let files = collect_files(project.path.clone());
  assert_eq!(files.len(), 6);
  std::env::set_current_dir(original_cwd).unwrap();
}