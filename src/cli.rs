use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    name    = "alpress",
    version = "0.1.0",
    about   = "Adaptive file compressor — automatically picks the best algorithm"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Compress a file using the best algorithm for its content
    Compress {
        /// Path to the file you want to compress
        #[arg(value_name = "INPUT")]
        input: String,

        /// Where to write the compressed output (default: <input>.alp)
        #[arg(value_name = "OUTPUT")]
        output: Option<String>,

        /// Show file profile before compressing
        #[arg(short, long)]
        verbose: bool,
    },

    /// Decompress an .alp file and restore the original
    Decompress {
        /// Path to the .alp file to decompress
        #[arg(value_name = "INPUT")]
        input: String,

        /// Where to write the restored file (default: strips .alp extension)
        #[arg(value_name = "OUTPUT")]
        output: Option<String>,
    },

    /// Analyze a file and show what Alpress would do — without compressing
    Analyze {
        /// Path to the file to analyze
        #[arg(value_name = "INPUT")]
        input: String,
    },

    /// Compare all algorithms on a file and show a performance table
    Benchmark {
        /// Path to the file to benchmark
        #[arg(value_name = "INPUT")]
        input: String,
    },
}