// stats.rs — Phase 1: File Statistics

pub fn print_stats(_file_path: &str, bytes: &[u8]) {
    print_size(bytes);
    println!();

    let freq = byte_frequency(bytes);
    print_top_bytes(&freq, 5);
    println!();

    let entropy = shannon_entropy(bytes);
    print_entropy(entropy);
    println!();

    print_compression_hint(entropy);
}

fn print_size(bytes: &[u8]) {
    let size = bytes.len();
    println!("📁 File Size");
    println!("   {} bytes", size);
    if size >= 1024 {
        println!("   {:.2} KB", size as f64 / 1024.0);
    }
    if size >= 1024 * 1024 {
        println!("   {:.2} MB", size as f64 / (1024.0 * 1024.0));
    }
}

fn byte_frequency(bytes: &[u8]) -> [u64; 256] {
    let mut freq = [0u64; 256];
    for &byte in bytes {
        freq[byte as usize] += 1;
    }
    freq
}

fn print_top_bytes(freq: &[u64; 256], n: usize) {
    println!("🔢 Byte Frequency (top {} most common)", n);
    let mut pairs: Vec<(usize, u64)> = freq
        .iter()
        .enumerate()
        .map(|(i, &count)| (i, count))
        .collect();
    pairs.sort_by(|a, b| b.1.cmp(&a.1));
    for (byte_val, count) in pairs.iter().take(n) {
        let display = byte_display(*byte_val as u8);
        println!("   Byte {:3} ({}) — {} times", byte_val, display, count);
    }
    let unique = freq.iter().filter(|&&c| c > 0).count();
    println!("   ({} unique byte values out of 256 possible)", unique);
}

fn byte_display(byte: u8) -> String {
    match byte {
        b'\n' => "newline".to_string(),
        b'\r' => "carriage return".to_string(),
        b'\t' => "tab".to_string(),
        b' '  => "space".to_string(),
        32..=126 => format!("'{}'", byte as char),
        _ => "non-printable".to_string(),
    }
}

pub fn unique_byte_count(bytes: &[u8]) -> usize {
    let freq = byte_frequency(bytes);
    freq.iter().filter(|&&c| c > 0).count()
}

pub fn shannon_entropy(bytes: &[u8]) -> f64 {
    if bytes.is_empty() {
        return 0.0;
    }
    let total = bytes.len() as f64;
    let freq = byte_frequency(bytes);
    freq.iter()
        .filter(|&&count| count > 0)
        .map(|&count| {
            let p = count as f64 / total;
            -p * p.log2()
        })
        .sum()
}

fn print_entropy(entropy: f64) {
    println!("📊 Shannon Entropy: {:.4} / 8.0000", entropy);
    let bar_width = 40;
    let filled = ((entropy / 8.0) * bar_width as f64).round() as usize;
    let filled = filled.min(bar_width);
    let bar: String = "█".repeat(filled) + &"░".repeat(bar_width - filled);
    println!("   [{}]", bar);
}

fn print_compression_hint(entropy: f64) {
    println!("💡 Compression Hint");
    let hint = if entropy < 1.0 {
        "Very low entropy — extremely compressible"
    } else if entropy < 3.5 {
        "Low entropy — highly compressible"
    } else if entropy < 5.5 {
        "Medium entropy — good compression expected"
    } else if entropy < 7.0 {
        "High entropy — moderate compression"
    } else if entropy < 7.5 {
        "Very high entropy — poor compression"
    } else {
        "Near-maximum entropy — skip compression"
    };
    println!("   {}", hint);
}