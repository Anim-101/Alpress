// main.rs — Phase 5: The Decision Engine

mod stats;
mod analyzer;
mod compressors;
mod errors;
mod selector; // NEW

use selector::AlgorithmSelector;
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

            // Phase 2: profile the file
            let profile = analyzer::FileProfile::analyze(&bytes);
            profile.print_summary();
            println!();

            // Phase 5: let the selector decide
            let selector = AlgorithmSelector::new();
            let decision = selector.select(&profile);
            decision.print_reasoning();
            println!();

            // Build and run the chosen compressor
            match selector.build_compressor(&decision.algorithm) {
                Ok(compressor) => {
                    println!("🗜️  Compressing with {}...\n", compressor.name());

                    match compressor.compress(&bytes) {
                        Ok(result) => {
                            result.print_summary();

                            // Round-trip check
                            match compressor.decompress(&result.data) {
                                Ok(restored) if restored == bytes =>
                                    println!("   🔁 Round-trip: ✅ PASSED"),
                                Ok(_) =>
                                    println!("   🔁 Round-trip: ❌ Data mismatch!"),
                                Err(e) =>
                                    println!("   🔁 Round-trip: ❌ {}", e),
                            }
                        }
                        Err(e) => eprintln!("❌ Compression failed: {}", e),
                    }
                }

                // AlreadyCompressed means the selector said "skip"
                Err(e) => {
                    println!("⏭️  {}", e);
                    println!("   File left as-is — no compression applied.");
                }
            }
        }
        Err(e) => {
            eprintln!("❌ Failed to read '{}': {}", file_path, e);
            process::exit(1);
        }
    }
}