pub mod collect;
pub mod scanner;
pub mod sloc;

use std::env::temp_dir;
use std::fs::remove_dir_all;
use std::path::PathBuf;
use std::sync::Mutex;

pub static CWD_LOCK: Mutex<()> = Mutex::new(());

pub struct ProjectBuf {
  pub path: PathBuf
}

impl ProjectBuf {
  pub fn new(name: &str) -> Self {
    let path = temp_dir().join(format!("{}-{}", name, std::process::id()));
    if path.exists() {
      remove_dir_all(&path).ok();
    }
    Self { path }
  }
  pub fn create_file(&self, name: &str) -> PathBuf {
    let path = self.path.join(name);
    std::fs::write(&path, "").ok();
    path
  }
  pub fn create_dir(&self, name: &str) {
    let path = self.path.join(name);
    std::fs::create_dir(path).ok();
  }
  pub fn create_file_with_content(&self, name: &str, content: &str) {
    let path = self.path.join(name);
    std::fs::write(path, content).ok();
  }
  pub fn generate_files(&self, count: usize) {
    for i in 0..count {
      self.create_file_with_content(&format!("file{}.txt", i), &format!("Line {}", i));
    }
  }
  pub fn create_dir_with_files(&self, name: &str, count: usize, ignore: Option<usize>) {
    let path = self.path.join(name);
    std::fs::create_dir(path).ok();
    for i in 0..count {
      self.create_file_with_content(&format!("{}/file{}.txt", name, i), &format!("Line {}", i));
    }
    if let Some(ignore) = ignore {
      for i in 0..ignore {
        self.create_file_with_content(&format!("{}/file{}.txt", name, i), &format!("Line {}", i));
      }
    }
  }
}

impl Drop for ProjectBuf {
  fn drop(&mut self) {
    if self.path.exists() {
      remove_dir_all(&self.path).ok();
    }
  }
}