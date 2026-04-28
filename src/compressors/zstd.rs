// compressors/zstd.rs — Phase 4: Zstandard Implementation
//
// Zstd is the MODERN sweet spot — made by Facebook (Meta):
//   - Better compression ratio than gzip at similar or faster speed
//   - Adjustable levels (1–22): level 3 is the general-purpose default
//   - Used by Linux kernel, Docker, Firefox, Android
//   - This is likely what Alpress will pick most of the time!

use super::traits::{Compressor, CompressionResult};
use crate::errors::{AlpressError, AlpressResult};

pub struct ZstdCompressor {
    level: i32, // 1 (fastest) to 22 (best compression)
}

impl ZstdCompressor {
    pub fn new(level: i32) -> Self {
        // Clamp level to valid range
        // LEARNING NOTE — .max() and .min() on numbers:
        //   These are methods on primitive types in Rust.
        //   level.max(1) ensures it's at least 1
        //   .min(22)     ensures it's at most 22
        Self { level: level.max(1).min(22) }
    }
}

impl Compressor for ZstdCompressor {
    fn name(&self) -> &str { "zstd" }

    fn compress(&self, data: &[u8]) -> AlpressResult<CompressionResult> {
        let original_size = data.len();

        let compressed = zstd::bulk::compress(data, self.level)
            .map_err(|e| AlpressError::CompressionFailed(e.to_string()))?;

        Ok(CompressionResult {
            compressed_size: compressed.len(),
            data: compressed,
            original_size,
            algorithm: format!("zstd-{}", self.level),
        })
    }

    fn decompress(&self, data: &[u8]) -> AlpressResult<Vec<u8>> {
        zstd::bulk::decompress(data, 100 * 1024 * 1024) // 100MB max output
            .map_err(|e| AlpressError::DecompressionFailed(e.to_string()))
    }
}