// sampler.rs — Phase 8: Smart File Sampling
//
// For large files, reading every byte just to decide HOW to compress
// is wasteful. Instead we take a representative sample:
//   - The start of the file (magic bytes, headers)
//   - A few chunks from the middle (body content)
//   - The end of the file (trailers, footers)
//
// This gives us a good statistical picture in a fraction of the time.
//
// LEARNING NOTE — slices and ranges:
//   &data[0..1024] takes the first 1024 bytes as a slice.
//   No copying — just a view into the original data.
//   This is one of Rust's core performance advantages.

/// How many bytes to sample from each region.
const REGION_SIZE: usize = 16 * 1024; // 16 KB per region

/// Files smaller than this are analyzed in full — no sampling needed.
const SAMPLING_THRESHOLD: usize = 3 * REGION_SIZE; // 48 KB

/// Describes where a sample came from — useful for debugging.
#[derive(Debug)]
pub struct SampleInfo {
    pub total_size:   usize,
    pub sample_size:  usize,
    pub was_sampled:  bool, // false = full file was used
}

impl SampleInfo {
    pub fn print(&self) {
        if self.was_sampled {
            println!(
                "   📐 Sampled {:.1} KB from {:.1} MB file ({:.1}% analyzed)",
                self.sample_size as f64 / 1024.0,
                self.total_size  as f64 / (1024.0 * 1024.0),
                (self.sample_size as f64 / self.total_size as f64) * 100.0,
            );
        } else {
            println!(
                "   📐 Full file analyzed ({:.1} KB)",
                self.total_size as f64 / 1024.0,
            );
        }
    }
}

/// Extract a representative sample from raw bytes.
///
/// Returns (sample_bytes, info) where sample_bytes is either:
///   - A reference to the full data (small files)
///   - A new Vec containing bytes from start + middle + end (large files)
///
/// LEARNING NOTE — returning an enum to avoid allocation:
///   For small files we return a slice (no copy).
///   For large files we return an owned Vec (must copy).
///   We use Cow<'_, [u8]> — "Clone On Write" — which handles both cases.
///   But to keep things simple here, we just return Vec<u8> always.
pub fn sample(data: &[u8]) -> (Vec<u8>, SampleInfo) {
    let total_size = data.len();

    // Small file — use everything
    if total_size <= SAMPLING_THRESHOLD {
        return (
            data.to_vec(),
            SampleInfo {
                total_size,
                sample_size: total_size,
                was_sampled: false,
            },
        );
    }

    // Large file — sample start, middle chunks, and end
    let mut sample = Vec::with_capacity(REGION_SIZE * 4);

    // 1. Start of file (first 16 KB)
    let start_end = REGION_SIZE.min(total_size);
    sample.extend_from_slice(&data[0..start_end]);

    // 2. Two chunks from the middle
    // LEARNING NOTE — integer arithmetic for offsets:
    //   We calculate offsets as fractions of total_size.
    //   Using usize arithmetic avoids floating point imprecision.
    let quarter = total_size / 4;
    let half    = total_size / 2;

    let quarter_end = (quarter + REGION_SIZE).min(total_size);
    sample.extend_from_slice(&data[quarter..quarter_end]);

    let half_end = (half + REGION_SIZE).min(total_size);
    sample.extend_from_slice(&data[half..half_end]);

    // 3. End of file (last 16 KB)
    let end_start = total_size.saturating_sub(REGION_SIZE);
    sample.extend_from_slice(&data[end_start..total_size]);

    let sample_size = sample.len();

    (
        sample,
        SampleInfo {
            total_size,
            sample_size,
            was_sampled: true,
        },
    )
}

// ─── Tests ───────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn small_file_not_sampled() {
        let data = vec![0u8; 1024]; // 1 KB
        let (s, info) = sample(&data);
        assert!(!info.was_sampled);
        assert_eq!(s.len(), data.len());
    }

    #[test]
    fn large_file_is_sampled() {
        let data = vec![0u8; 1024 * 1024]; // 1 MB
        let (s, info) = sample(&data);
        assert!(info.was_sampled);
        assert!(s.len() < data.len());
    }

    #[test]
    fn sample_size_is_reasonable() {
        let data = vec![42u8; 10 * 1024 * 1024]; // 10 MB
        let (s, info) = sample(&data);
        assert!(info.was_sampled);
        // Sample should be well under 10% of the file
        assert!(s.len() < data.len() / 5);
    }
}