// src/main.rs
/*
 * Main executable for InterpreterPattern
 */

use clap::Parser;
use interpreterpattern::{Result, run};

#[derive(Parser)]
#[command(version, about = "InterpreterPattern - A Rust implementation")]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
    
    /// Input file path
    #[arg(short, long)]
    input: Option<String>,
    
    /// Output file path
    #[arg(short, long)]
    output: Option<String>,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose, args.input, args.output)
}
