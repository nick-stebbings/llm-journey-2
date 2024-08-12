// Import necessary crates and modules
extern crate dir_stats;
use dir_stats::DirStats;
use std::env;
use std::process;

// Main function
fn main() {
    // Collect command line arguments
    let args: Vec<String> = env::args().collect();
    // Check if the correct number of arguments are provided
    if args.len() != 2 {
        eprintln!("Usage: {} <path>", args[0]);
        process::exit(1);
    }

    // Create a new DirStats object
    let mut stats = DirStats::new();
    // Gather stats for the provided path
    let path = std::path::Path::new(&args[1]);
    if let Err(err) = stats.gather_stats(&path) {
        eprintln!("Error gathering stats: {}", err);
        process::exit(1);
    }

    // Print the gathered stats
    println!("File count: {}", stats.file_count);
    println!("Line count: {}", stats.line_count);
    println!("Commit count: {}", stats.commit_count);
}
