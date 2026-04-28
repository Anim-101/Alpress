// header.rs — Phase 6: Binary File Format

use std::io::{Read, Write};
use crate::errors::{AlpressError, AlpressResult};

pub const MAGIC: &[u8; 4] = b"ALPS";
pub const VERSION: u8 = 1;
pub const HEADER_SIZE: usize = 24;

#[derive(Debug, Clone, PartialEq)]
#[repr(u8)]
pub enum AlgorithmId {
    Gzip     = 0x01,
    GzipFast = 0x02,
    GzipBest = 0x03,
    Lz4      = 0x04,
    Zstd     = 0x05,
}

impl AlgorithmId {
    pub fn from_byte(byte: u8) -> AlpressResult<AlgorithmId> {
        match byte {
            0x01 => Ok(AlgorithmId::Gzip),
            0x02 => Ok(AlgorithmId::GzipFast),
            0x03 => Ok(AlgorithmId::GzipBest),
            0x04 => Ok(AlgorithmId::Lz4),
            0x05 => Ok(AlgorithmId::Zstd),
            other => Err(AlpressError::DecompressionFailed(
                format!("Unknown algorithm ID: 0x{:02X}", other)
            )),
        }
    }

    pub fn as_byte(&self) -> u8 {
        self.clone() as u8
    }

    pub fn name(&self) -> &str {
        match self {
            AlgorithmId::Gzip     => "gzip",
            AlgorithmId::GzipFast => "gzip-fast",
            AlgorithmId::GzipBest => "gzip-best",
            AlgorithmId::Lz4      => "lz4",
            AlgorithmId::Zstd     => "zstd",
        }
    }
}

#[derive(Debug)]
pub struct FileHeader {
    pub version:       u8,
    pub algorithm:     AlgorithmId,
    pub original_size: u64,
    pub checksum:      u32,
}

impl FileHeader {
    pub fn new(algorithm: AlgorithmId, original_data: &[u8]) -> FileHeader {
        let checksum = crc32fast::hash(original_data);
        FileHeader {
            version: VERSION,
            algorithm,
            original_size: original_data.len() as u64,
            checksum,
        }
    }

    pub fn write_to<W: Write>(&self, writer: &mut W) -> AlpressResult<()> {
        writer.write_all(MAGIC)
            .map_err(|e| AlpressError::Io(e.to_string()))?;
        writer.write_all(&[self.version])
            .map_err(|e| AlpressError::Io(e.to_string()))?;
        writer.write_all(&[self.algorithm.as_byte()])
            .map_err(|e| AlpressError::Io(e.to_string()))?;
        writer.write_all(&self.original_size.to_le_bytes())
            .map_err(|e| AlpressError::Io(e.to_string()))?;
        writer.write_all(&self.checksum.to_le_bytes())
            .map_err(|e| AlpressError::Io(e.to_string()))?;
        writer.write_all(&[0u8; 6])
            .map_err(|e| AlpressError::Io(e.to_string()))?;
        Ok(())
    }

    pub fn read_from<R: Read>(reader: &mut R) -> AlpressResult<FileHeader> {
        let mut buf = [0u8; HEADER_SIZE];
        reader.read_exact(&mut buf)
            .map_err(|e| AlpressError::DecompressionFailed(
                format!("Failed to read header: {}", e)
            ))?;

        if &buf[0..4] != MAGIC {
            return Err(AlpressError::DecompressionFailed(
                "Not an Alpress file — magic bytes missing".to_string()
            ));
        }

        let version = buf[4];
        if version != VERSION {
            return Err(AlpressError::DecompressionFailed(
                format!("Unsupported format version: {}", version)
            ));
        }

        let algorithm    = AlgorithmId::from_byte(buf[5])?;
        let original_size = u64::from_le_bytes(buf[6..14].try_into().unwrap());
        let checksum      = u32::from_le_bytes(buf[14..18].try_into().unwrap());

        Ok(FileHeader { version, algorithm, original_size, checksum })
    }

    pub fn verify(&self, data: &[u8]) -> AlpressResult<()> {
        if data.len() as u64 != self.original_size {
            return Err(AlpressError::DecompressionFailed(format!(
                "Size mismatch: expected {} bytes, got {}",
                self.original_size, data.len()
            )));
        }
        let actual = crc32fast::hash(data);
        if actual != self.checksum {
            return Err(AlpressError::DecompressionFailed(format!(
                "Checksum mismatch: expected 0x{:08X}, got 0x{:08X}",
                self.checksum, actual
            )));
        }
        Ok(())
    }
}