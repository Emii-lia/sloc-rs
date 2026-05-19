pub struct IgnoreRules {
  starts_with: &'static [&'static str],
  ends_with: &'static [&'static str],
  contains: &'static [&'static str],
  exact: &'static [&'static str],
}

pub const IGNORE_FILE: IgnoreRules = IgnoreRules {
  starts_with: &[".", "_"],
  ends_with: &[".log", ".tmp", ".bak", ".lock", ".md"],
  contains: &[],
  exact: &[],
};

pub const IGNORE_DIR: IgnoreRules = IgnoreRules {
  starts_with: &[".", "_"],
  ends_with: &[],
  contains: &[],
  exact: &[
    ".git", ".idea", ".vscode", ".cache", ".output", "node_modules",
    "target", "dist", "build", "vendor", "logs", "test", "tests", "public"
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