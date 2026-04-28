// compressors/mod.rs — Phase 4: updated with lz4 + zstd

pub mod traits;
pub mod gzip;
pub mod lz4;   // NEW
pub mod zstd;  // NEW

pub use traits::{Compressor, CompressionResult};