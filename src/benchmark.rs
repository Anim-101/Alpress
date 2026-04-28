use std::time::Instant;

use crate::compressors::Compressor;
use crate::compressors::gzip::{GzipCompressor, GzipLevel};
use crate::compressors::lz4::Lz4Compressor;
use crate::compressors::zstd::ZstdCompressor;
use crate::errors::AlpressResult;

pub struct BenchmarkRow {
    pub algorithm:       String,
    pub original_size:   usize,
    pub compressed_size: usize,
    pub ratio:           f64,
    pub duration_ms:     f64,
}

pub fn run_benchmark(data: &[u8]) -> AlpressResult<Vec<BenchmarkRow>> {
    let compressors: Vec<Box<dyn Compressor>> = vec![
        Box::new(GzipCompressor::new(GzipLevel::Fast)),
        Box::new(GzipCompressor::new(GzipLevel::Default)),
        Box::new(GzipCompressor::new(GzipLevel::Best)),
        Box::new(Lz4Compressor::new()),
        Box::new(ZstdCompressor::new(1)),
        Box::new(ZstdCompressor::new(3)),
        Box::new(ZstdCompressor::new(9)),
        Box::new(ZstdCompressor::new(19)),
    ];

    let mut rows = Vec::new();

    for compressor in &compressors {
        let start   = Instant::now();
        let result  = compressor.compress(data)?;
        let elapsed = start.elapsed();

        let ratio = 1.0 - (result.compressed_size as f64 / result.original_size as f64);

        rows.push(BenchmarkRow {
            algorithm:       result.algorithm.clone(),
            original_size:   result.original_size,
            compressed_size: result.compressed_size,
            ratio,
            duration_ms:     elapsed.as_secs_f64() * 1000.0,
        });
    }

    // Sort by compressed size — best compression first
    rows.sort_by(|a, b| a.compressed_size.cmp(&b.compressed_size));

    Ok(rows)
}

pub fn print_benchmark(rows: &[BenchmarkRow]) {
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("📊  BENCHMARK RESULTS  (sorted by compressed size)");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("{:<14} {:>12} {:>12} {:>10} {:>10}",
        "Algorithm", "Original", "Compressed", "Saved", "Time"
    );
    println!("{}", "─".repeat(62));

    for (i, row) in rows.iter().enumerate() {
        let marker = if i == 0 { "🏆" } else { "  " };
        println!("{} {:<12} {:>12} {:>12} {:>9.1}% {:>8.1}ms",
            marker,
            row.algorithm,
            format_bytes(row.original_size),
            format_bytes(row.compressed_size),
            row.ratio * 100.0,
            row.duration_ms,
        );
    }

    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

    if let Some(winner) = rows.first() {
        println!("🏆  Best ratio  : {} ({:.1}% smaller)",
            winner.algorithm, winner.ratio * 100.0);
    }

    if let Some(fastest) = rows.iter().min_by(|a, b| {
        a.duration_ms.partial_cmp(&b.duration_ms).unwrap()
    }) {
        println!("⚡  Fastest     : {} ({:.1}ms)",
            fastest.algorithm, fastest.duration_ms);
    }

    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
}

fn format_bytes(bytes: usize) -> String {
    if bytes < 1024 {
        format!("{} B", bytes)
    } else if bytes < 1024 * 1024 {
        format!("{:.1} KB", bytes as f64 / 1024.0)
    } else {
        format!("{:.1} MB", bytes as f64 / (1024.0 * 1024.0))
    }
}