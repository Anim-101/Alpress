// selector.rs — Phase 5: The Decision Engine
//
// This is the brain of Alpress.
// It reads a FileProfile and decides which algorithm to use —
// so the user never has to think about it.
//
// LEARNING NOTE — this is where all our previous phases pay off:
//   Phase 1 gave us entropy
//   Phase 2 gave us FileProfile + FileType
//   Phase 3 gave us the Compressor trait
//   Phase 4 gave us multiple algorithms + AlpressError
//   Phase 5 wires them all together into one intelligent selector

use crate::analyzer::FileProfile;
use crate::errors::{AlpressError, AlpressResult};
use crate::compressors::Compressor;
use crate::compressors::gzip::{GzipCompressor, GzipLevel};
use crate::compressors::lz4::Lz4Compressor;
use crate::compressors::zstd::ZstdCompressor;

// ─── Algorithm Enum ──────────────────────────────────────────────────────────
//
// LEARNING NOTE — enums WITH data:
//   Each variant can carry its own data.
//   Algorithm::Zstd { level: 3 } holds a level inside it.
//   Algorithm::Skip { reason } holds a string explaining why.
//   This is one of Rust's most powerful features!

#[derive(Debug, Clone)]
pub enum Algorithm {
    Gzip { level: GzipLevel },
    Lz4,
    Zstd { level: i32 },

    // Sometimes the smartest move is NOT to compress
    Skip { reason: String },
}

impl Algorithm {
    /// Human-readable name of the chosen algorithm.
    pub fn name(&self) -> String {
        match self {
            Algorithm::Gzip { level } => match level {
                GzipLevel::Fast    => "gzip-fast".to_string(),
                GzipLevel::Default => "gzip".to_string(),
                GzipLevel::Best    => "gzip-best".to_string(),
            },
            Algorithm::Lz4           => "lz4".to_string(),
            Algorithm::Zstd { level } => format!("zstd-{}", level),
            Algorithm::Skip { reason } => format!("skip ({})", reason),
        }
    }
}

// ─── Selection Decision ──────────────────────────────────────────────────────

/// The full output of the selector — not just the algorithm,
/// but also WHY it was chosen.
///
/// LEARNING NOTE — carrying reasoning with decisions:
///   Good systems are explainable. Instead of just returning an Algorithm,
///   we return a Decision that includes the reasoning chain.
///   This makes debugging and user feedback much easier.
pub struct Decision {
    pub algorithm: Algorithm,
    pub reasoning: Vec<String>, // ordered list of reasons
}

impl Decision {
    /// Print why this decision was made — great for transparency.
    pub fn print_reasoning(&self) {
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        println!("🧠  ALGORITHM DECISION");
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        println!("   Chosen       : {}", self.algorithm.name());
        println!("   Reasoning    :");
        for (i, reason) in self.reasoning.iter().enumerate() {
            println!("   {}. {}", i + 1, reason);
        }
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    }
}

// ─── AlgorithmSelector ───────────────────────────────────────────────────────

/// Reads a FileProfile and decides the best compression strategy.
pub struct AlgorithmSelector;

impl AlgorithmSelector {
    pub fn new() -> Self { Self }

    /// The main decision function.
    /// Takes a profile, returns a Decision explaining what to do and why.
    pub fn select(&self, profile: &FileProfile) -> Decision {
        let mut reasoning = Vec::new();

        // ── Rule 1: Already compressed? Skip entirely ──────────────────────
        if profile.file_type.is_already_compressed() {
            reasoning.push(format!(
                "File type '{}' is already compressed",
                profile.file_type.description()
            ));
            reasoning.push("Compressing again would likely make it larger".to_string());
            return Decision {
                algorithm: Algorithm::Skip {
                    reason: "already compressed".to_string(),
                },
                reasoning,
            };
        }

        // ── Rule 2: Very high entropy? Not worth compressing ───────────────
        if profile.entropy > 7.5 {
            reasoning.push(format!(
                "Entropy is very high ({:.2}) — data appears random or encrypted",
                profile.entropy
            ));
            reasoning.push("Compression would produce little to no gain".to_string());
            return Decision {
                algorithm: Algorithm::Skip {
                    reason: "entropy too high".to_string(),
                },
                reasoning,
            };
        }

        // ── Rule 3: Tiny files — Gzip default is fine ─────────────────────
        // Small files don't benefit from heavy algorithms
        if profile.size_bytes < 4 * 1024 {
            reasoning.push(format!(
                "File is very small ({} bytes) — lightweight compression preferred",
                profile.size_bytes
            ));
            reasoning.push("Gzip default balances overhead vs gain for small files".to_string());
            return Decision {
                algorithm: Algorithm::Gzip { level: GzipLevel::Default },
                reasoning,
            };
        }

        // ── Rule 4: Very compressible — use Zstd high ─────────────────────
        if profile.compressibility_score >= 0.75 {
            reasoning.push(format!(
                "Compressibility score is excellent ({:.0}%)",
                profile.compressibility_score * 100.0
            ));
            reasoning.push(format!(
                "Entropy is low ({:.2}) — data has strong patterns",
                profile.entropy
            ));
            reasoning.push("Zstd level 19 will achieve maximum compression ratio".to_string());
            return Decision {
                algorithm: Algorithm::Zstd { level: 19 },
                reasoning,
            };
        }

        // ── Rule 5: Good compressibility — Zstd default ───────────────────
        if profile.compressibility_score >= 0.50 {
            reasoning.push(format!(
                "Compressibility score is good ({:.0}%)",
                profile.compressibility_score * 100.0
            ));
            reasoning.push("Zstd level 3 gives great ratio with fast speed".to_string());
            reasoning.push("Best general-purpose choice for this file".to_string());
            return Decision {
                algorithm: Algorithm::Zstd { level: 3 },
                reasoning,
            };
        }

        // ── Rule 6: Large file with moderate compressibility — use LZ4 ────
        // Speed matters more than ratio for large files
        if profile.size_bytes > 50 * 1024 * 1024 {
            reasoning.push(format!(
                "File is large ({:.1} MB)",
                profile.size_bytes as f64 / (1024.0 * 1024.0)
            ));
            reasoning.push(format!(
                "Moderate compressibility ({:.0}%) — prioritize speed over ratio",
                profile.compressibility_score * 100.0
            ));
            reasoning.push("LZ4 gives the fastest compression for large files".to_string());
            return Decision {
                algorithm: Algorithm::Lz4,
                reasoning,
            };
        }

        // ── Rule 7: Low compressibility — Gzip fast ───────────────────────
        if profile.compressibility_score >= 0.25 {
            reasoning.push(format!(
                "Compressibility is low ({:.0}%) — heavy algorithms won't help much",
                profile.compressibility_score * 100.0
            ));
            reasoning.push("Gzip fast gives a small gain without wasting CPU".to_string());
            return Decision {
                algorithm: Algorithm::Gzip { level: GzipLevel::Fast },
                reasoning,
            };
        }

        // ── Rule 8: Fallback — skip ────────────────────────────────────────
        reasoning.push(format!(
            "Compressibility score too low ({:.0}%) — not worth compressing",
            profile.compressibility_score * 100.0
        ));
        reasoning.push("Storing file as-is is the best option".to_string());
        Decision {
            algorithm: Algorithm::Skip {
                reason: "compressibility too low".to_string(),
            },
            reasoning,
        }
    }

    /// Build the actual Compressor from a Decision.
    ///
    /// LEARNING NOTE — Box<dyn Compressor>:
    ///   We return a trait object here so callers don't need to
    ///   know which concrete type they're getting — just that it
    ///   implements Compressor. This is runtime polymorphism in Rust.
    pub fn build_compressor(
        &self,
        algorithm: &Algorithm,
    ) -> AlpressResult<Box<dyn Compressor>> {
        match algorithm {
            Algorithm::Gzip { level } =>
                Ok(Box::new(GzipCompressor::new(level.clone()))),
            Algorithm::Lz4 =>
                Ok(Box::new(Lz4Compressor::new())),
            Algorithm::Zstd { level } =>
                Ok(Box::new(ZstdCompressor::new(*level))),

            // LEARNING NOTE — matching a variant with data:
            //   `Algorithm::Skip { reason }` destructures the variant,
            //   binding the inner String to `reason`.
            Algorithm::Skip { reason } =>
                Err(AlpressError::AlreadyCompressed(
                    format!("Skipping compression: {}", reason)
                )),
        }
    }
}

// ─── Tests ───────────────────────────────────────────────────────────────────
//
// LEARNING NOTE — unit tests in Rust:
//   Tests live right inside the same file, in a `#[cfg(test)]` block.
//   `#[cfg(test)]` means "only compile this when running tests".
//   Each test function is marked with `#[test]`.
//   Run all tests with: cargo test

#[cfg(test)]
mod tests {
    use super::*;
    use crate::analyzer::FileProfile;
    use crate::analyzer::file_type::FileType;

    // Helper to build a fake profile for testing
    fn make_profile(
        entropy: f64,
        file_type: FileType,
        size_bytes: u64,
        compressibility_score: f64,
    ) -> FileProfile {
        FileProfile {
            size_bytes,
            entropy,
            file_type,
            byte_diversity: 0.5,
            compressibility_score,
        }
    }

    #[test]
    fn test_already_compressed_skips() {
        let profile = make_profile(7.9, FileType::Jpeg, 1024 * 100, 0.05);
        let selector = AlgorithmSelector::new();
        let decision = selector.select(&profile);

        // LEARNING NOTE — matches! macro in tests:
        //   We use matches! to check the enum variant without
        //   having to write a full match block.
        assert!(matches!(decision.algorithm, Algorithm::Skip { .. }));
    }

    #[test]
    fn test_high_entropy_skips() {
        let profile = make_profile(7.9, FileType::Binary, 1024 * 100, 0.10);
        let selector = AlgorithmSelector::new();
        let decision = selector.select(&profile);
        assert!(matches!(decision.algorithm, Algorithm::Skip { .. }));
    }

    #[test]
    fn test_highly_compressible_uses_zstd_high() {
        let profile = make_profile(1.0, FileType::PlainText, 1024 * 100, 0.90);
        let selector = AlgorithmSelector::new();
        let decision = selector.select(&profile);
        assert!(matches!(decision.algorithm, Algorithm::Zstd { level: 19 }));
    }

    #[test]
    fn test_good_compressibility_uses_zstd_default() {
        let profile = make_profile(3.0, FileType::SourceCode, 1024 * 50, 0.60);
        let selector = AlgorithmSelector::new();
        let decision = selector.select(&profile);
        assert!(matches!(decision.algorithm, Algorithm::Zstd { level: 3 }));
    }

    #[test]
    fn test_small_file_uses_gzip() {
        let profile = make_profile(3.0, FileType::PlainText, 1024, 0.60);
        let selector = AlgorithmSelector::new();
        let decision = selector.select(&profile);
        assert!(matches!(decision.algorithm, Algorithm::Gzip { .. }));
    }

    #[test]
    fn test_reasoning_is_not_empty() {
        let profile = make_profile(3.0, FileType::PlainText, 1024 * 50, 0.60);
        let selector = AlgorithmSelector::new();
        let decision = selector.select(&profile);
        assert!(!decision.reasoning.is_empty());
    }
}