// compressors/lz4.rs — Phase 4: LZ4 Implementation
//
// LZ4 is all about SPEED:
//   - Fastest compression algorithm in common use
//   - Compression ratio is lower than gzip/zstd
//   - Perfect for large files where speed matters more than size
//   - Used internally by Linux kernel, databases, game engines

use super::traits::{Compressor, CompressionResult};
use crate::errors::{AlpressError, AlpressResult};

pub struct Lz4Compressor;

// LEARNING NOTE — unit structs:
//   `Lz4Compressor` has no fields — it holds no configuration.
//   This is called a "unit struct". It still gets methods via impl.
//   We instantiate it as just: Lz4Compressor  (no braces needed)

impl Lz4Compressor {
    pub fn new() -> Self { Self }
}

impl Compressor for Lz4Compressor {
    fn name(&self) -> &str { "lz4" }

    fn compress(&self, data: &[u8]) -> AlpressResult<CompressionResult> {
        let original_size = data.len();

        let compressed = lz4::block::compress(data, None, true)
            .map_err(|e| AlpressError::CompressionFailed(e.to_string()))?;

        Ok(CompressionResult {
            compressed_size: compressed.len(),
            data: compressed,
            original_size,
            algorithm: self.name().to_string(),
        })
    }

    fn decompress(&self, data: &[u8]) -> AlpressResult<Vec<u8>> {
        // LEARNING NOTE — LZ4 needs the original size hint to decompress.
        //   The lz4 crate stores it in the first 4 bytes when `store_size: true`.
        //   We pass a safe upper bound — it won't actually allocate that much.
        lz4::block::decompress(data, Some(i32::MAX))
            .map_err(|e| AlpressError::DecompressionFailed(e.to_string()))
    }
}