// main.rs — Phase 6: Real file compression + decompression

mod stats;
mod analyzer;
mod compressors;
mod errors;
mod selector;
mod header;  // NEW
mod archive; // NEW

use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        print_usage(&args[0]);
        process::exit(1);
    }

    let command   = &args[1];
    let input     = &args[2];

    match command.as_str() {
        // ── compress ──────────────────────────────────────────────────────
        "compress" => {
            // Output path: either provided or auto-generated (.alp)
            let output = if args.len() >= 4 {
                args[3].clone()
            } else {
                format!("{}.alp", input)
            };

            println!("\n🗜️  Compressing '{}' → '{}'\n", input, output);

            match archive::compress_file(input, &output) {
                Ok(summary) => summary.print(),
                Err(e)      => {
                    eprintln!("❌ {}", e);
                    process::exit(1);
                }
            }
        }

        // ── decompress ────────────────────────────────────────────────────
        "decompress" => {
            // Output path: either provided or strip .alp extension
            let output = if args.len() >= 4 {
                args[3].clone()
            } else if input.ends_with(".alp") {
                input.trim_end_matches(".alp").to_string()
            } else {
                format!("{}.restored", input)
            };

            println!("\n📂 Decompressing '{}' → '{}'\n", input, output);

            match archive::decompress_file(input, &output) {
                Ok(summary) => summary.print(),
                Err(e)      => {
                    eprintln!("❌ {}", e);
                    process::exit(1);
                }
            }
        }

        // ── analyze ───────────────────────────────────────────────────────
        // Still useful for inspecting a file before compressing
        "analyze" => {
            match std::fs::read(input) {
                Ok(bytes) => {
                    println!("\n✅ Read: {}  ({} bytes)\n", input, bytes.len());
                    let profile = analyzer::FileProfile::analyze(&bytes);
                    profile.print_summary();
                    println!();
                    let selector = selector::AlgorithmSelector::new();
                    let decision = selector.select(&profile);
                    decision.print_reasoning();
                }
                Err(e) => {
                    eprintln!("❌ Cannot read '{}': {}", input, e);
                    process::exit(1);
                }
            }
        }

        _ => {
            eprintln!("❌ Unknown command: '{}'", command);
            print_usage(&args[0]);
            process::exit(1);
        }
    }
}

fn print_usage(program: &str) {
    println!("\nAlpress — Adaptive File Compressor");
    println!();
    println!("USAGE:");
    println!("   {} compress   <input>        [output.alp]", program);
    println!("   {} decompress <input.alp>    [output]", program);
    println!("   {} analyze    <input>", program);
    println!();
    println!("EXAMPLES:");
    println!("   {} compress   src/main.rs", program);
    println!("   {} decompress src/main.rs.alp", program);
    println!("   {} analyze    my_file.log", program);
}