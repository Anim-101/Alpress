// compressors/traits.rs — Phase 4: Updated with custom error type

use crate::errors::AlpressResult;

/// Result of a compression operation.
#[derive(Debug)]
pub struct CompressionResult {
    pub data:             Vec<u8>,
    pub original_size:    usize,
    pub compressed_size:  usize,
    pub algorithm:        String,
}

impl CompressionResult {
    pub fn ratio(&self) -> f64 {
        if self.original_size == 0 { return 0.0; }
        1.0 - (self.compressed_size as f64 / self.original_size as f64)
    }

    pub fn print_summary(&self) {
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        println!("🗜️   COMPRESSION RESULT");
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        println!("   Algorithm    : {}", self.algorithm);
        println!("   Before       : {} bytes", self.original_size);
        println!("   After        : {} bytes", self.compressed_size);
        println!("   Ratio        : {:.1}% smaller", self.ratio() * 100.0);
        if self.ratio() > 0.0 {
            println!("   Verdict      : ✅ Compression helped!");
        } else {
            println!("   Verdict      : ⚠️  File got larger!");
        }
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    }
}

/// Every compression algorithm must implement this trait.
pub trait Compressor {
    fn name(&self) -> &str;

    // LEARNING NOTE — AlpressResult vs Box<dyn Error>:
    //   Phase 3 used Box<dyn Error> — anything could be an error.
    //   Now we use AlpressResult<T> which means ONLY AlpressError can fail.
    //   This is more precise and better communicates intent to callers.
    fn compress(&self, data: &[u8]) -> AlpressResult<CompressionResult>;
    fn decompress(&self, data: &[u8]) -> AlpressResult<Vec<u8>>;
}