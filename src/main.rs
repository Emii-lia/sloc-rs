use std::path::PathBuf;
use std::time::Instant;
use clap::Parser;
use crate::cli::Args;
use crate::commands::sloc;
use crate::types::Config;

pub mod walker;
pub mod types;
pub mod scanner;
pub mod utils;
pub mod commands;
pub mod cli;
pub mod test;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let start = Instant::now();
    let args = Args::parse();
    let (stats, file_len) = sloc(&Config {
        root: if args.root == "." {
            std::env::current_dir()?
        } else { PathBuf::from(args.root) },
        threads: args.threads
    });

    let elapsed = start.elapsed();

    println!("Lines of code: {}", stats.code);
    println!("Blank lines: {}", stats.blank);
    println!("Files: {}", file_len);
    println!("Elapsed time: {}ms", elapsed.as_millis());
    println!("Done!");

    Ok(())
}
