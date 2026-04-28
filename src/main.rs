// main.rs — Phase 4: All 3 algorithms + proper error handling

mod stats;
mod analyzer;
mod compressors;
mod errors; // NEW

use compressors::Compressor;
use compressors::gzip::{GzipCompressor, GzipLevel};
use compressors::lz4::Lz4Compressor;
use compressors::zstd::ZstdCompressor;
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

            // Phase 2: profile
            let profile = analyzer::FileProfile::analyze(&bytes);
            profile.print_summary();
            println!();

            // Phase 4: try ALL algorithms and compare!
            // LEARNING NOTE — Vec<Box<dyn Compressor>>:
            //   We want a list of different compressor types.
            //   Since they're different types (Gzip, Lz4, Zstd),
            //   we use Box<dyn Compressor> — a trait object.
            //   Each box can hold ANY type that implements Compressor.
            let compressors: Vec<Box<dyn Compressor>> = vec![
                Box::new(GzipCompressor::new(GzipLevel::Default)),
                Box::new(GzipCompressor::new(GzipLevel::Best)),
                Box::new(Lz4Compressor::new()),
                Box::new(ZstdCompressor::new(3)),   // fast default
                Box::new(ZstdCompressor::new(19)),  // high compression
            ];

            println!("🗜️  Running all algorithms...\n");

            // Keep track of the best result for a summary at the end
            let mut best_ratio: f64 = f64::NEG_INFINITY;
            let mut best_algo = String::new();

            for compressor in &compressors {
                match compressor.compress(&bytes) {
                    Ok(result) => {
                        result.print_summary();

                        // Track the best ratio
                        if result.ratio() > best_ratio {
                            best_ratio = result.ratio();
                            best_algo = result.algorithm.clone();
                        }

                        // Round-trip check
                        match compressor.decompress(&result.data) {
                            Ok(restored) if restored == bytes =>
                                println!("   🔁 Round-trip: ✅ PASSED\n"),
                            Ok(_) =>
                                println!("   🔁 Round-trip: ❌ Data mismatch!\n"),
                            // LEARNING NOTE — matching on our custom error:
                            //   Now that we have AlpressError, we can match
                            //   on specific variants to handle each case.
                            Err(e) =>
                                println!("   🔁 Round-trip: ❌ {}\n", e),
                        }
                    }
                    // LEARNING NOTE — using our custom error in a match:
                    //   e is an AlpressError — we can match its variants
                    //   and respond differently to each failure mode.
                    Err(e) => eprintln!("❌ [{}] Failed: {}\n", compressor.name(), e),
                }
            }

            // Final winner summary
            println!("🏆 Best algorithm: {} ({:.1}% smaller)", best_algo, best_ratio * 100.0);
        }
        Err(e) => {
            eprintln!("❌ Failed to read '{}': {}", file_path, e);
            process::exit(1);
        }
    }
}