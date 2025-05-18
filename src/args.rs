// src/args.rs
use clap::Parser;
use std::path::PathBuf;

/// vidcrop <input> <start> <end>
#[derive(Parser)]
#[command(name = "vidcrop", about = "Trim videos via ffmpeg")]
pub struct Args {
    /// Input video file
    #[arg(value_parser = clap::value_parser!(PathBuf))]
    pub input: PathBuf,

    /// Trim start (e.g. 90s, 1m30s, HH:MM:SS or MM:SS)
    pub start: String,

    /// Trim end (same format as start)
    pub end: String,
}
