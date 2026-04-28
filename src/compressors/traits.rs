// compressors/traits.rs — Phase 3: The Compressor Trait
//
// LEARNING NOTE — Traits:
//   A trait is a contract. It says:
//   "Any type that implements this trait MUST have these methods."
//
//   Think of it like an interface in Java/TypeScript, or an
//   abstract base class in Python — but more powerful.
//
//   Later, GzipCompressor, Lz4Compressor, ZstdCompressor will
//   ALL implement this same trait. That means we can write code
//   that works with ANY compressor without knowing which one it is.

/// The result of a compression operation — holds useful metadata.
///
/// LEARNING NOTE — Structs as return values:
///   Instead of returning just Vec<u8>, we return a richer struct
///   so callers can see compression ratio, algorithm used, etc.
#[derive(Debug)]
pub struct CompressionResult {
    /// The compressed bytes
    pub data: Vec<u8>,

    /// Original size in bytes
    pub original_size: usize,

    /// Compressed size in bytes
    pub compressed_size: usize,

    /// Which algorithm was used (e.g. "gzip", "lz4")
    pub algorithm: String,
}

impl CompressionResult {
    /// How much smaller did we make it? (0.0 = no gain, 1.0 = perfect)
    pub fn ratio(&self) -> f64 {
        if self.original_size == 0 {
            return 0.0;
        }
        // e.g. 1000 bytes → 400 bytes = 0.60 (60% smaller)
        1.0 - (self.compressed_size as f64 / self.original_size as f64)
    }

    /// Print a summary of the compression result.
    pub fn print_summary(&self) {
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        println!("🗜️   COMPRESSION RESULT");
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        println!("   Algorithm    : {}", self.algorithm);
        println!("   Before       : {} bytes", self.original_size);
        println!("   After        : {} bytes", self.compressed_size);
        println!("   Ratio        : {:.1}% smaller", self.ratio() * 100.0);

        // Was it actually worth it?
        if self.ratio() > 0.0 {
            println!("   Verdict      : ✅ Compression helped!");
        } else {
            println!("   Verdict      : ⚠️  File got larger — compression hurt!");
        }

        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    }
}

// ─── The Compressor Trait ────────────────────────────────────────────────────

/// Any compression algorithm must implement this trait.
///
/// LEARNING NOTE — trait definition:
///   `pub trait Name { fn method(...); }` declares the trait.
///   Types implement it with `impl Trait for Type { ... }`.
///
/// LEARNING NOTE — Box<dyn std::error::Error>:
///   This means "any error type". We'll replace this with our own
///   custom error type in Phase 4. For now it keeps things simple.
pub trait Compressor {
    /// The name of this algorithm (e.g. "gzip", "lz4").
    fn name(&self) -> &str;

    /// Compress raw bytes. Returns a CompressionResult or an error.
    fn compress(&self, data: &[u8]) -> Result<CompressionResult, Box<dyn std::error::Error>>;

    /// Decompress bytes that were compressed by this algorithm.
    fn decompress(&self, data: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>>;
}