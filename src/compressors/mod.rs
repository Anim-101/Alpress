// compressors/mod.rs

pub mod traits;
pub mod gzip;
pub mod lz4;
pub mod zstd;

pub use traits::Compressor;