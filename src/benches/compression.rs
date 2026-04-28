// benches/compression.rs — Phase 8: Criterion Benchmarks
//
// LEARNING NOTE — criterion:
//   `cargo test` runs correctness tests.
//   `cargo bench` runs performance benchmarks using criterion.
//   Criterion runs each function many times, measures variance,
//   and produces a statistical report — much more reliable than
//   measuring with Instant::now() manually.
//
// Run with: cargo bench
// Results appear in: target/criterion/

use criterion::{black_box, criterion_group, criterion_main, Criterion};

// We need to access our compressors from the bench binary.
// Since alpress is a [[bin]] not a [[lib]], we reference the source directly.
use alpress_bench::*;

// LEARNING NOTE — black_box():
//   Prevents the compiler from optimizing away benchmark code.
//   Without it, the compiler might see "this result is unused"
//   and skip the work entirely, making benchmarks useless.

fn make_test_data(size: usize) -> Vec<u8> {
    // Realistic text-like data: mix of repeated patterns
    let pattern = b"Hello world! This is a test of Alpress compression. ";
    let mut data = Vec::with_capacity(size);
    while data.len() < size {
        let remaining = size - data.len();
        let chunk = &pattern[..remaining.min(pattern.len())];
        data.extend_from_slice(chunk);
    }
    data
}

fn bench_gzip(c: &mut Criterion) {
    let data = make_test_data(100 * 1024); // 100 KB

    c.bench_function("gzip_compress_100kb", |b| {
        b.iter(|| {
            use flate2::Compression;
            use flate2::write::GzEncoder;
            use std::io::Write;
            let mut enc = GzEncoder::new(Vec::new(), Compression::default());
            enc.write_all(black_box(&data)).unwrap();
            enc.finish().unwrap()
        })
    });
}

fn bench_lz4(c: &mut Criterion) {
    let data = make_test_data(100 * 1024);

    c.bench_function("lz4_compress_100kb", |b| {
        b.iter(|| {
            lz4::block::compress(black_box(&data), None, true).unwrap()
        })
    });
}

fn bench_zstd(c: &mut Criterion) {
    let data = make_test_data(100 * 1024);

    c.bench_function("zstd_compress_100kb", |b| {
        b.iter(|| {
            zstd::bulk::compress(black_box(&data), 3).unwrap()
        })
    });
}

criterion_group!(benches, bench_gzip, bench_lz4, bench_zstd);
criterion_main!(benches);