// archive.rs — Phase 6: Reading and Writing .alp files

use std::fs;
use std::io::BufWriter;

use crate::errors::{AlpressError, AlpressResult};
use crate::header::{AlgorithmId, FileHeader, HEADER_SIZE};
use crate::analyzer::FileProfile;
use crate::selector::{Algorithm, AlgorithmSelector};
use crate::compressors::gzip::GzipLevel;
use crate::compressors::Compressor;
use crate::compressors::gzip::GzipCompressor;
use crate::compressors::lz4::Lz4Compressor;
use crate::compressors::zstd::ZstdCompressor;

pub fn compress_file(input_path: &str, output_path: &str) -> AlpressResult<CompressSummary> {
    let original = fs::read(input_path)
        .map_err(|e| AlpressError::Io(format!("Cannot read '{}': {}", input_path, e)))?;

    let profile  = FileProfile::analyze(&original);
    let selector = AlgorithmSelector::new();
    let decision = selector.select(&profile);

    let (compressor, algo_id): (Box<dyn Compressor>, AlgorithmId) =
        match &decision.algorithm {
            Algorithm::Gzip { level } => (
                Box::new(GzipCompressor::new(level.clone())),
                match level {
                    GzipLevel::Fast    => AlgorithmId::GzipFast,
                    GzipLevel::Default => AlgorithmId::Gzip,
                    GzipLevel::Best    => AlgorithmId::GzipBest,
                }
            ),
            Algorithm::Lz4 => (
                Box::new(Lz4Compressor::new()),
                AlgorithmId::Lz4,
            ),
            Algorithm::Zstd { level } => (
                Box::new(ZstdCompressor::new(*level)),
                AlgorithmId::Zstd,
            ),
            Algorithm::Skip { reason } => {
                return Err(AlpressError::AlreadyCompressed(
                    format!("Skipping: {}", reason)
                ));
            }
        };

    let result = compressor.compress(&original)?;
    let header = FileHeader::new(algo_id, &original);

    let output_file = fs::File::create(output_path)
        .map_err(|e| AlpressError::Io(format!("Cannot create '{}': {}", output_path, e)))?;

    let mut writer = BufWriter::new(output_file);
    header.write_to(&mut writer)?;

    use std::io::Write;
    writer.write_all(&result.data)
        .map_err(|e| AlpressError::Io(e.to_string()))?;

    Ok(CompressSummary {
        algorithm:       result.algorithm,
        original_size:   result.original_size,
        compressed_size: result.compressed_size,
        output_path:     output_path.to_string(),
        reasoning:       decision.reasoning,
    })
}

pub fn decompress_file(input_path: &str, output_path: &str) -> AlpressResult<DecompressSummary> {
    let alp_data = fs::read(input_path)
        .map_err(|e| AlpressError::Io(format!("Cannot read '{}': {}", input_path, e)))?;

    let mut reader = std::io::Cursor::new(&alp_data);
    let header     = FileHeader::read_from(&mut reader)?;

    let compressed_data = &alp_data[HEADER_SIZE..];

    let decompressed = match &header.algorithm {
        AlgorithmId::Gzip     => GzipCompressor::new(GzipLevel::Default).decompress(compressed_data)?,
        AlgorithmId::GzipFast => GzipCompressor::new(GzipLevel::Fast).decompress(compressed_data)?,
        AlgorithmId::GzipBest => GzipCompressor::new(GzipLevel::Best).decompress(compressed_data)?,
        AlgorithmId::Lz4      => Lz4Compressor::new().decompress(compressed_data)?,
        AlgorithmId::Zstd     => ZstdCompressor::new(3).decompress(compressed_data)?,
    };

    header.verify(&decompressed)?;

    fs::write(output_path, &decompressed)
        .map_err(|e| AlpressError::Io(format!("Cannot write '{}': {}", output_path, e)))?;

    Ok(DecompressSummary {
        algorithm:         header.algorithm.name().to_string(),
        compressed_size:   compressed_data.len(),
        restored_size:     decompressed.len(),
        output_path:       output_path.to_string(),
        checksum_verified: true,
    })
}

pub struct CompressSummary {
    pub algorithm:       String,
    pub original_size:   usize,
    pub compressed_size: usize,
    pub output_path:     String,
    pub reasoning:       Vec<String>,
}

impl CompressSummary {
    pub fn print(&self) {
        let ratio = 1.0 - (self.compressed_size as f64 / self.original_size as f64);
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        println!("✅  COMPRESSED SUCCESSFULLY");
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        println!("   Algorithm  : {}", self.algorithm);
        println!("   Before     : {} bytes", self.original_size);
        println!("   After      : {} bytes", self.compressed_size);
        println!("   Saved      : {:.1}%", ratio * 100.0);
        println!("   Output     : {}", self.output_path);
        println!("   Reasoning  :");
        for (i, r) in self.reasoning.iter().enumerate() {
            println!("   {}. {}", i + 1, r);
        }
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    }
}

pub struct DecompressSummary {
    pub algorithm:         String,
    pub compressed_size:   usize,
    pub restored_size:     usize,
    pub output_path:       String,
    pub checksum_verified: bool,
}

impl DecompressSummary {
    pub fn print(&self) {
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        println!("✅  DECOMPRESSED SUCCESSFULLY");
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        println!("   Algorithm  : {}", self.algorithm);
        println!("   Compressed : {} bytes", self.compressed_size);
        println!("   Restored   : {} bytes", self.restored_size);
        println!("   Output     : {}", self.output_path);
        println!("   Integrity  : {}",
            if self.checksum_verified { "✅ CRC32 verified" } else { "⚠️ Not verified" }
        );
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    }
}