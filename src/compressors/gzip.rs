// compressors/gzip.rs — Phase 4: Updated Gzip

use flate2::Compression;
use flate2::write::GzEncoder;
use flate2::read::GzDecoder;
use std::io::{Write, Read};

use super::traits::{Compressor, CompressionResult};
use crate::errors::{AlpressError, AlpressResult};

#[derive(Debug, Clone)]
pub enum GzipLevel { Fast, Default, Best }

pub struct GzipCompressor { level: GzipLevel }

impl GzipCompressor {
    pub fn new(level: GzipLevel) -> Self { Self { level } }

    fn compression_level(&self) -> Compression {
        match &self.level {
            GzipLevel::Fast    => Compression::fast(),
            GzipLevel::Default => Compression::default(),
            GzipLevel::Best    => Compression::best(),
        }
    }
}

impl Compressor for GzipCompressor {
    fn name(&self) -> &str {
        match &self.level {
            GzipLevel::Fast    => "gzip-fast",
            GzipLevel::Default => "gzip",
            GzipLevel::Best    => "gzip-best",
        }
    }

    fn compress(&self, data: &[u8]) -> AlpressResult<CompressionResult> {
        let original_size = data.len();
        let mut encoder = GzEncoder::new(Vec::new(), self.compression_level());

        // LEARNING NOTE — map_err():
        //   `?` alone would work if the error types matched.
        //   But std::io::Error ≠ AlpressError, so we convert it first.
        //   .map_err(|e| AlpressError::CompressionFailed(e.to_string()))
        //   transforms the error type before propagating with `?`.
        encoder.write_all(data)
            .map_err(|e| AlpressError::CompressionFailed(e.to_string()))?;
        let compressed = encoder.finish()
            .map_err(|e| AlpressError::CompressionFailed(e.to_string()))?;

        Ok(CompressionResult {
            compressed_size: compressed.len(),
            data: compressed,
            original_size,
            algorithm: self.name().to_string(),
        })
    }

    fn decompress(&self, data: &[u8]) -> AlpressResult<Vec<u8>> {
        let mut decoder = GzDecoder::new(data);
        let mut out = Vec::new();
        decoder.read_to_end(&mut out)
            .map_err(|e| AlpressError::DecompressionFailed(e.to_string()))?;
        Ok(out)
    }
}