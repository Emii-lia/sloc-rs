const SUPPORTED_FILE_EXTENSIONS: &[&str] = &[
  "json", "yaml", "yml", "md", "txt", "html", "css", "scss", "js", "sh", "py", "rs", "toml", "json", "yaml", "yml", "md", "txt", "html", "css", "js", "sh", "py",
  "ts", "tsx", "jsx", "java", "c", "cpp", "h", "hpp", "go", "rb", "php", "swift", "kt", "kts", "scala", "lua", "sql", "xml", "csv", "tsv", "ini", "conf", "cfg", "log", "lock", "env", "properties"
];

pub fn is_supported_file_extension(path: &str) -> bool {
  SUPPORTED_FILE_EXTENSIONS.iter().any(|ext| path.ends_with(ext))
}