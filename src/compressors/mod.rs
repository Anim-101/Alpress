// compressors/mod.rs — Phase 3: Compressor module entry point
//
// This module will grow every phase:
//   Phase 3 → gzip
//   Phase 4 → lz4, zstd
//   Phase 5 → selector picks between them

pub mod traits;  // the Compressor trait
pub mod gzip;    // our first implementation

pub use traits::{Compressor, CompressionResult};