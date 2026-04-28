// errors.rs — Phase 4: Custom Error Types
//
// LEARNING NOTE — Why custom errors?
//   So far we used Box<dyn std::error::Error> — it works but it's vague.
//   A custom error enum lets callers know EXACTLY what went wrong
//   and handle each case differently if they want.
//
// LEARNING NOTE — enum variants with data:
//   Unlike Phase 2's simple enums (FileType::Jpeg),
//   these variants carry a String message explaining what failed.
//   This is a very common Rust pattern.

/// All the ways Alpress can fail.
#[derive(Debug)]
pub enum AlpressError {
    /// Something went wrong reading or writing a file
    Io(String),

    /// Compression failed
    CompressionFailed(String),

    /// Decompression failed (bad data, wrong algorithm, etc.)
    DecompressionFailed(String),

    /// The file is already compressed — no point compressing again
    AlreadyCompressed(String),
}

// LEARNING NOTE — implementing Display:
//   To print an error with `{}` (not just `{:?}`), we implement
//   the `std::fmt::Display` trait. This is what gets shown to users.
impl std::fmt::Display for AlpressError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AlpressError::Io(msg) =>
                write!(f, "I/O error: {}", msg),
            AlpressError::CompressionFailed(msg) =>
                write!(f, "Compression failed: {}", msg),
            AlpressError::DecompressionFailed(msg) =>
                write!(f, "Decompression failed: {}", msg),
            AlpressError::AlreadyCompressed(msg) =>
                write!(f, "Already compressed: {}", msg),
        }
    }
}

// LEARNING NOTE — implementing the Error trait:
//   To be used as a proper Rust error, our type must implement
//   std::error::Error. The default implementation is empty — that's fine.
impl std::error::Error for AlpressError {}

// LEARNING NOTE — From trait for automatic conversion:
//   By implementing From<std::io::Error> for AlpressError,
//   the `?` operator can automatically convert std::io::Error
//   into our AlpressError::Io variant. Very ergonomic!
impl From<std::io::Error> for AlpressError {
    fn from(e: std::io::Error) -> Self {
        AlpressError::Io(e.to_string())
    }
}

/// A Result type pre-filled with our error — saves typing!
///
/// LEARNING NOTE — type aliases:
///   Instead of writing Result<T, AlpressError> everywhere,
///   we define `AlpressResult<T>` as a shorthand.
///   The standard library does the same with std::io::Result<T>.
pub type AlpressResult<T> = Result<T, AlpressError>;