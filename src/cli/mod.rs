use clap::{Parser};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
  pub root: String,

  #[arg(short, long, help = "The number of threads to use")]
  pub threads: Option<usize>,
}