pub struct IgnoreRules {
  starts_with: &'static [&'static str],
  ends_with: &'static [&'static str],
  contains: &'static [&'static str],
  exact: &'static [&'static str],
}

pub const IGNORE_FILE: IgnoreRules = IgnoreRules {
  starts_with: &[".", "_", ".env"],
  ends_with: &[".log", ".tmp", ".bak", ".lock", ".md", ".css.map"],
  contains: &["LICENSE"],
  exact: &["package-lock.json", "yarn.lock", "pnpm-lock.yaml", "bun.lockb"],
};

pub const IGNORE_DIR: IgnoreRules = IgnoreRules {
  starts_with: &[".", "_"],
  ends_with: &[],
  contains: &[],
  exact: &[
    ".git", ".idea", ".vscode", ".cache", ".output", "node_modules",
    "target", "dist", "build", "vendor", "logs", "test", "tests", "public", "logs",
    "static", "assets", "media", "uploads", "tmp", "temp", "backup", "backups",
  ],
};

impl IgnoreRules {
  pub fn is_ignored(&self, path: &str) -> bool {
    self.starts_with.iter().any(|rule| path.starts_with(rule))
      || self.ends_with.iter().any(|rule| path.ends_with(rule))
      || self.contains.iter().any(|rule| path.contains(rule))
      || self.exact.iter().any(|rule| path == *rule)
  }
}