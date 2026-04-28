// analyzer/file_type.rs — File Type Detection via magic bytes

#[derive(Debug, Clone, PartialEq)]
pub enum FileType {
    Jpeg, Png, Gif, WebP,
    Zip, Gzip, Zstd, Lz4, Bzip2, Xz,
    Pdf, Mp4, Mp3,
    PlainText, SourceCode, Binary, Unknown,
}

impl FileType {
    pub fn detect(bytes: &[u8]) -> FileType {
        if bytes.len() < 4 { return FileType::Unknown; }

        if bytes.starts_with(&[0xFF, 0xD8, 0xFF])              { return FileType::Jpeg; }
        if bytes.starts_with(&[0x89, 0x50, 0x4E, 0x47])        { return FileType::Png; }
        if bytes.starts_with(b"GIF87a") || bytes.starts_with(b"GIF89a") { return FileType::Gif; }
        if bytes.starts_with(b"RIFF") && bytes.len() >= 12 && &bytes[8..12] == b"WEBP" { return FileType::WebP; }
        if bytes.starts_with(&[0x50, 0x4B, 0x03, 0x04])        { return FileType::Zip; }
        if bytes.starts_with(&[0x1F, 0x8B])                    { return FileType::Gzip; }
        if bytes.starts_with(&[0x28, 0xB5, 0x2F, 0xFD])        { return FileType::Zstd; }
        if bytes.starts_with(&[0x04, 0x22, 0x4D, 0x18])        { return FileType::Lz4; }
        if bytes.starts_with(b"BZh")                           { return FileType::Bzip2; }
        if bytes.starts_with(&[0xFD, 0x37, 0x7A, 0x58, 0x5A, 0x00]) { return FileType::Xz; }
        if bytes.starts_with(b"%PDF")                          { return FileType::Pdf; }
        if bytes.len() >= 8 && &bytes[4..8] == b"ftyp"        { return FileType::Mp4; }
        if bytes.starts_with(&[0xFF, 0xFB]) || bytes.starts_with(b"ID3") { return FileType::Mp3; }

        classify_by_content(bytes)
    }

    pub fn is_already_compressed(&self) -> bool {
        matches!(self,
            FileType::Jpeg | FileType::Png  | FileType::Gif  | FileType::WebP |
            FileType::Zip  | FileType::Gzip | FileType::Zstd | FileType::Lz4  |
            FileType::Bzip2 | FileType::Xz | FileType::Mp4  | FileType::Mp3
        )
    }

    pub fn description(&self) -> &str {
        match self {
            FileType::Jpeg       => "JPEG Image",
            FileType::Png        => "PNG Image",
            FileType::Gif        => "GIF Image",
            FileType::WebP       => "WebP Image",
            FileType::Zip        => "ZIP Archive",
            FileType::Gzip       => "Gzip Compressed",
            FileType::Zstd       => "Zstandard Compressed",
            FileType::Lz4        => "LZ4 Compressed",
            FileType::Bzip2      => "Bzip2 Compressed",
            FileType::Xz         => "XZ Compressed",
            FileType::Pdf        => "PDF Document",
            FileType::Mp4        => "MP4 Video",
            FileType::Mp3        => "MP3 Audio",
            FileType::PlainText  => "Plain Text",
            FileType::SourceCode => "Source Code",
            FileType::Binary     => "Binary Data",
            FileType::Unknown    => "Unknown",
        }
    }
}

fn classify_by_content(bytes: &[u8]) -> FileType {
    let sample = &bytes[..bytes.len().min(1024)];
    let printable = sample.iter()
        .filter(|&&b| b == b'\n' || b == b'\r' || b == b'\t' || (32..=126).contains(&b))
        .count();
    let ratio = printable as f64 / sample.len() as f64;
    if ratio > 0.85 {
        if looks_like_source_code(sample) { FileType::SourceCode } else { FileType::PlainText }
    } else {
        FileType::Binary
    }
}

fn looks_like_source_code(bytes: &[u8]) -> bool {
    let text = String::from_utf8_lossy(bytes);
    let indicators = [
        "fn ", "let ", "pub ", "use ",
        "def ", "import ", "class ",
        "int ", "void ", "return ",
        "const ", "function ", "var ",
        "#include", "struct ",
    ];
    indicators.iter().any(|&pat| text.contains(pat))
}