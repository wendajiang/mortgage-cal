use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "cal")]
#[command(author = "david")]
#[command(version = "0.1")]
#[command(about = "Calculate the mortgage and analyze.", long_about = None)]
pub struct Cli {
    #[arg(short, long = "gen")]
    pub generate: bool,
    #[arg(short, long, value_name = "FILE")]
    pub config: Option<PathBuf>,
    #[arg(short, long = "template")]
    pub template: bool,
}
