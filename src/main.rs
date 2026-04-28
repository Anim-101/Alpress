// main.rs — Phase 3: First Real Compression!

mod stats;
mod analyzer;
mod compressors; // NEW

use std::env;
use std::process;
use compressors::{Compressor, CompressionResult};
use compressors::gzip::{GzipCompressor, GzipLevel};

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

            // Phase 3: compress with Gzip and show results
            // We'll try all three levels so you can see the difference!
            println!("🗜️  Compressing with Gzip...\n");

            let levels = vec![
                GzipCompressor::new(GzipLevel::Fast),
                GzipCompressor::new(GzipLevel::Default),
                GzipCompressor::new(GzipLevel::Best),
            ];

            for compressor in &levels {
                match compressor.compress(&bytes) {
                    Ok(result) => {
                        result.print_summary();

                        // Sanity check: decompress and verify it matches
                        match compressor.decompress(&result.data) {
                            Ok(restored) => {
                                if restored == bytes {
                                    println!("   🔁 Round-trip check: ✅ PASSED");
                                } else {
                                    println!("   🔁 Round-trip check: ❌ FAILED (data mismatch!)");
                                }
                            }
                            Err(e) => println!("   🔁 Decompression error: {}", e),
                        }
                        println!();
                    }
                    Err(e) => eprintln!("❌ Compression failed: {}", e),
                }
            }
        }
        Err(e) => {
            eprintln!("❌ Failed to read '{}': {}", file_path, e);
            process::exit(1);
        }
    }
}