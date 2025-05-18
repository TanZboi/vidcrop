mod args;
mod parser;
mod ffmpeg;

use anyhow::{anyhow, Result};
use clap::Parser;
use std::path::PathBuf;

fn main() -> Result<()> {
    // 1) Parse the CLI
    let args = args::Args::parse();

    // 2) Ensure input file exists
    let input_path: PathBuf = args.input.clone();
    if !input_path.exists() {
        return Err(anyhow!("input file {:?} does not exist", input_path));
    }

    // 3) Convert PathBuf → &str
    let input_str = input_path
        .to_str()
        .ok_or_else(|| anyhow!("invalid input filename"))?;

    // 4) Parse & validate times, mapping String errors into anyhow::Error
    let start = parser::parse_time(&args.start)
        .map_err(|e| anyhow!(e))?;
    let end = parser::parse_time(&args.end)
        .map_err(|e| anyhow!(e))?;
    parser::validate_range(start, end)
        .map_err(|e| anyhow!(e))?;

    // 5) Derive output filename
    let output = ffmpeg::derive_output(input_str);

    // 6) Run ffmpeg (frame-accurate re-encode)
    ffmpeg::run(input_str, start, end, &output)?;

    println!("Wrote '{}'", output);
    Ok(())
}
