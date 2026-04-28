// main.rs — Phase 7: Full CLI with clap

mod stats;
mod analyzer;
mod compressors;
mod errors;
mod selector;
mod header;
mod archive;
mod cli;
mod benchmark;

use clap::Parser;
use cli::{Cli, Commands};

fn main() {
    let cli = Cli::parse();

    match cli.command {

        // ── compress ──────────────────────────────────────────────────────
        Commands::Compress { input, output, verbose } => {
            let output_path = output.unwrap_or_else(|| format!("{}.alp", input));

            println!("\n🗜️  Alpress — Compressing");
            println!("   Input  : {}", input);
            println!("   Output : {}\n", output_path);

            if verbose {
                match std::fs::read(&input) {
                    Ok(bytes) => {
                        let profile = analyzer::FileProfile::analyze(&bytes);
                        profile.print_summary();
                        println!();
                        let selector = selector::AlgorithmSelector::new();
                        let decision = selector.select(&profile);
                        decision.print_reasoning();
                        println!();
                    }
                    Err(e) => eprintln!("⚠️  Could not profile file: {}", e),
                }
            }

            match archive::compress_file(&input, &output_path) {
                Ok(summary) => summary.print(),
                Err(e) => {
                    eprintln!("❌ {}", e);
                    std::process::exit(1);
                }
            }
        }

        // ── decompress ────────────────────────────────────────────────────
        Commands::Decompress { input, output } => {
            let output_path = output.unwrap_or_else(|| {
                if input.ends_with(".alp") {
                    input.trim_end_matches(".alp").to_string()
                } else {
                    format!("{}.restored", input)
                }
            });

            println!("\n📂 Alpress — Decompressing");
            println!("   Input  : {}", input);
            println!("   Output : {}\n", output_path);

            match archive::decompress_file(&input, &output_path) {
                Ok(summary) => summary.print(),
                Err(e) => {
                    eprintln!("❌ {}", e);
                    std::process::exit(1);
                }
            }
        }

        // ── analyze ───────────────────────────────────────────────────────
        Commands::Analyze { input } => {
            println!("\n🔍 Alpress — Analyzing: {}\n", input);

            match std::fs::read(&input) {
                Ok(bytes) => {
                    stats::print_stats(&input, &bytes);
                    println!();
                    let profile = analyzer::FileProfile::analyze(&bytes);
                    profile.print_summary();
                    println!();
                    let selector = selector::AlgorithmSelector::new();
                    let decision = selector.select(&profile);
                    decision.print_reasoning();
                }
                Err(e) => {
                    eprintln!("❌ Cannot read '{}': {}", input, e);
                    std::process::exit(1);
                }
            }
        }

        // ── benchmark ─────────────────────────────────────────────────────
        Commands::Benchmark { input } => {
            println!("\n⚡ Alpress — Benchmarking: {}\n", input);

            match std::fs::read(&input) {
                Ok(bytes) => {
                    println!("Running all algorithms on {} bytes...\n", bytes.len());
                    match benchmark::run_benchmark(&bytes) {
                        Ok(rows) => benchmark::print_benchmark(&rows),
                        Err(e)   => eprintln!("❌ Benchmark failed: {}", e),
                    }
                }
                Err(e) => {
                    eprintln!("❌ Cannot read '{}': {}", input, e);
                    std::process::exit(1);
                }
            }
        }

    } // end match cli.command
}   // end fn main