use std::process::Command;
use anyhow::{Context, Result};
use clap::Parser;

#[derive(Parser)]
#[command(
    name = "vidcrop",
    about = "A simple video trimmer using ffmpeg"
)]
struct Args {
    /// Input video file
    #[clap(short, long)]
    input: String,

    /// Trim start time (HH:MM:SS)
    #[clap(short, long)]
    start: String,

    /// Trim end time (HH:MM:SS)
    #[clap(short, long)]
    end: String,

    /// Output file
    #[clap(short, long)]
    output: String,
}

fn main() -> Result<()> {
    let args = Args::parse();

    // Build and run the ffmpeg command
    let status = Command::new("ffmpeg")
        .args(&[
            "-i", &args.input,
            "-ss", &args.start,
            "-to", &args.end,
            "-c", "copy",
            &args.output,
        ])
        .status()
        .context("failed to spawn ffmpeg process")?;

    if !status.success() {
        anyhow::bail!("ffmpeg exited with status: {}", status);
    }

    println!("Trimmed '{}' from {} to {} → '{}'",
             &args.input, &args.start, &args.end, &args.output);

    Ok(())
}
