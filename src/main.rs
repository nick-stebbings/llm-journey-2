extern crate dir_stats;

use dir_stats::DirStats;
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <path>", args[0]);
        process::exit(1);
    }

    let mut stats = DirStats::new();
    if let Err(err) = stats.gather_stats(&args[1]) {
        eprintln!("Error gathering stats: {}", err);
        process::exit(1);
    }

    println!("File count: {}", stats.file_count);
    println!("Line count: {}", stats.line_count);
    println!("Commit count: {}", stats.commit_count);
}
