// main.rs — Phase 2: File Profiler

mod stats;
mod analyzer;

use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage:   {} <file_path>", args[0]);
        eprintln!("Example: {} src/main.rs", args[0]);
        process::exit(1);
    }

    let file_path = &args[1];

    match std::fs::read(file_path) {
        Ok(bytes) => {
            println!("\n✅ Read: {}  ({} bytes)\n", file_path, bytes.len());

            // Phase 1: raw stats
            stats::print_stats(file_path, &bytes);
            println!();

            // Phase 2: structured profile
            let profile = analyzer::FileProfile::analyze(&bytes);
            profile.print_summary();
        }
        Err(e) => {
            eprintln!("❌ Failed to read '{}': {}", file_path, e);
            process::exit(1);
        }
    }
}