// compressors/gzip.rs — Phase 3: Gzip Implementation
//
// Gzip is a great starting point:
//   - Available everywhere, well understood
//   - Good compression for text files
//   - The `flate2` crate wraps the underlying C library safely
//
// LEARNING NOTE — implementing a trait:
//   `impl Compressor for GzipCompressor` means:
//   "GzipCompressor now satisfies the Compressor contract."
//   The compiler will ERROR if we forget any required method.

use flate2::Compression;
use flate2::write::GzEncoder;
use flate2::read::GzDecoder;
use std::io::Write;
use std::io::Read;

use super::traits::{Compressor, CompressionResult};

// ─── Compression Level ───────────────────────────────────────────────────────

/// How hard should Gzip try to compress?
///
/// LEARNING NOTE — enums with meaning:
///   Instead of passing magic numbers (1, 6, 9) around,
///   we give them names. This makes code much more readable.
#[derive(Debug, Clone)]
pub enum GzipLevel {
    Fast,    // level 1 — quick but less compression
    Default, // level 6 — balanced (what most tools use)
    Best,    // level 9 — maximum compression, slower
}

// ─── GzipCompressor Struct ───────────────────────────────────────────────────

/// A compressor that uses the Gzip algorithm.
///
/// LEARNING NOTE — structs as objects:
///   This struct holds configuration (which level to use).
///   Methods are added via `impl`. This is Rust's version of a class.
pub struct GzipCompressor {
    level: GzipLevel,
}

impl GzipCompressor {
    /// Create a new GzipCompressor with a given level.
    pub fn new(level: GzipLevel) -> Self {
        // LEARNING NOTE — `Self`:
        //   Inside an impl block, `Self` means "this type" (GzipCompressor).
        //   It's the same as writing `GzipCompressor { level }`.
        Self { level }
    }

    /// Convert our GzipLevel enum to the value flate2 expects.
    fn compression_level(&self) -> Compression {
        // LEARNING NOTE — match on &self.level:
        //   We borrow self.level (don't move it) using &
        //   Each arm maps our enum variant to flate2's Compression type
        match &self.level {
            GzipLevel::Fast    => Compression::fast(),
            GzipLevel::Default => Compression::default(),
            GzipLevel::Best    => Compression::best(),
        }
    }
}

// ─── Implementing the Compressor Trait ───────────────────────────────────────

impl Compressor for GzipCompressor {
    fn name(&self) -> &str {
        // Return a different name depending on the level
        // so output is informative
        match &self.level {
            GzipLevel::Fast    => "gzip-fast",
            GzipLevel::Default => "gzip",
            GzipLevel::Best    => "gzip-best",
        }
    }

    fn compress(&self, data: &[u8]) -> Result<CompressionResult, Box<dyn std::error::Error>> {
        let original_size = data.len();

        // GzEncoder is a "writer" that compresses as you write to it.
        // We write into a Vec<u8> buffer.
        //
        // LEARNING NOTE — Vec::new() as a writer:
        //   Many Rust types implement Write/Read traits.
        //   Vec<u8> implements Write — you can write bytes into it.
        let mut encoder = GzEncoder::new(Vec::new(), self.compression_level());

        // Write all the input data into the encoder
        // The `?` at the end is Rust's error propagation operator:
        //   if this returns Err(...), the whole function returns that Err
        //   if it returns Ok(...), we get the inner value
        encoder.write_all(data)?;

        // Flush and finalize — this produces the complete gzip stream
        let compressed = encoder.finish()?;
        let compressed_size = compressed.len();

        Ok(CompressionResult {
            data: compressed,
            original_size,
            compressed_size,
            algorithm: self.name().to_string(),
        })
    }

    fn decompress(&self, data: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        // GzDecoder is a "reader" that decompresses as you read from it.
        let mut decoder = GzDecoder::new(data);
        let mut decompressed = Vec::new();

        // Read all decompressed bytes into our Vec
        decoder.read_to_end(&mut decompressed)?;

        Ok(decompressed)
    }
}