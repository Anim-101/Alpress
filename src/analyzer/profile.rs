// analyzer/profile.rs — FileProfile struct

use super::file_type::FileType;
use crate::stats;

#[derive(Debug, Clone)]
pub struct FileProfile {
    pub size_bytes:           u64,
    pub entropy:              f64,
    pub file_type:            FileType,
    pub byte_diversity:       f64,
    pub compressibility_score: f64,
}

impl FileProfile {
    pub fn analyze(bytes: &[u8]) -> FileProfile {
        let size_bytes          = bytes.len() as u64;
        let entropy             = stats::shannon_entropy(bytes);
        let file_type           = FileType::detect(bytes);
        let unique_bytes        = stats::unique_byte_count(bytes);
        let byte_diversity      = unique_bytes as f64 / 256.0;
        let compressibility_score = estimate_compressibility(entropy, &file_type, byte_diversity);

        FileProfile { size_bytes, entropy, file_type, byte_diversity, compressibility_score }
    }

    pub fn print_summary(&self) {
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        println!("📋  FILE PROFILE");
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        println!("📁  Size            : {}", format_size(self.size_bytes));
        println!("🔍  File Type       : {}", self.file_type.description());
        println!("📊  Entropy         : {:.4} / 8.0", self.entropy);
        println!("    {}", entropy_bar(self.entropy));
        println!("🎲  Byte Diversity  : {:.1}%  ({} unique byte values)",
            self.byte_diversity * 100.0,
            (self.byte_diversity * 256.0).round() as usize
        );
        println!("💡  Compressibility : {:.0}%  — {}",
            self.compressibility_score * 100.0,
            compressibility_label(self.compressibility_score)
        );
        if self.file_type.is_already_compressed() {
            println!();
            println!("⚠️   Already compressed — skipping would be smarter.");
        }
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    }
}

fn estimate_compressibility(entropy: f64, file_type: &FileType, byte_diversity: f64) -> f64 {
    if file_type.is_already_compressed() { return 0.05; }
    let entropy_score   = 1.0 - (entropy / 8.0);
    let diversity_score = 1.0 - byte_diversity;
    let score = (entropy_score * 0.70) + (diversity_score * 0.30);
    score.max(0.0_f64).min(1.0_f64)
}

fn compressibility_label(score: f64) -> &'static str {
    if score >= 0.80      { "Excellent — very high compression ratio" }
    else if score >= 0.60 { "Good — solid compression expected" }
    else if score >= 0.40 { "Moderate — some compression benefit" }
    else if score >= 0.20 { "Poor — minimal compression gain" }
    else                  { "Skip — compression would make it larger" }
}

fn format_size(bytes: u64) -> String {
    if bytes < 1024           { format!("{} B", bytes) }
    else if bytes < 1024*1024 { format!("{:.2} KB ({} bytes)", bytes as f64 / 1024.0, bytes) }
    else                      { format!("{:.2} MB ({} bytes)", bytes as f64 / (1024.0*1024.0), bytes) }
}

fn entropy_bar(entropy: f64) -> String {
    let width  = 40;
    let filled = ((entropy / 8.0) * width as f64).round() as usize;
    let filled = filled.min(width);
    format!("[{}{}]", "█".repeat(filled), "░".repeat(width - filled))
}