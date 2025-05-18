use std::{fs, path::Path, process::Command};
use anyhow::{Context, Result};
use clap::Parser;
use humantime::Duration as HumanDuration;
use std::time::Duration;

/// A simple video trimmer using ffmpeg (frame-accurate via re-encode)
#[derive(Parser)]
#[command(name = "vidcrop", about = "Trim videos via ffmpeg")]
struct Args {
    /// Input video file
    #[clap(short, long, value_parser = existing_file)]
    input: String,

    /// Trim start time (e.g. 90s, 1m30s, HH:MM:SS or MM:SS)
    #[clap(short, long, value_parser = parse_human_time)]
    start: Duration,

    /// Trim end time (same format as start)
    #[clap(short, long, value_parser = parse_human_time)]
    end: Duration,

    /// Output file (will be overwritten if exists)
    #[clap(short, long)]
    output: String,
}

fn main() -> Result<()> {
    let args = Args::parse();

    // 1. Validate start < end
    if args.start >= args.end {
        anyhow::bail!(
            "Start time ({:?}) must be less than end time ({:?})",
            args.start,
            args.end
        );
    }

    // 2. Ensure output directory exists
    if let Some(dir) = Path::new(&args.output).parent() {
        fs::create_dir_all(dir)
            .with_context(|| format!("could not create directory {:?}", dir))?;
    }

    // Helper: seconds → "HH:MM:SS"
    let fmt = |d: Duration| {
        let secs = d.as_secs();
        let h = secs / 3600;
        let m = (secs % 3600) / 60;
        let s = secs % 60;
        format!("{:02}:{:02}:{:02}", h, m, s)
    };

    // 3. Prepare timestamps
    let start_ts = fmt(args.start);

    // Compute exact clip duration (end – start) and format for -t
    let duration = args
        .end
        .checked_sub(args.start)
        .expect("start < end ensured above");
    let dur_ts = fmt(duration);

    // 4. Frame-accurate, re-encode invocation:
    //    -ss <start_ts>  → seek to exactly start_ts
    //    -i <input>      → read input
    //    -t  <dur_ts>    → transcode exactly duration seconds
    //    -c:v libx264    → re-encode video
    //    -c:a aac        → re-encode audio
    //    -y              → overwrite without asking
    let status = Command::new("ffmpeg")
        .args(&[
            "-ss",  &start_ts,
            "-i",   &args.input,
            "-t",   &dur_ts,
            "-c:v", "libx264",
            "-c:a", "aac",
            "-y",
            &args.output,
        ])
        .status()
        .context("failed to spawn ffmpeg process")?;

    if !status.success() {
        anyhow::bail!("ffmpeg exited with status: {}", status);
    }

    println!(
        "Trimmed '{}' from {} to {} ({}s) → '{}'",
        &args.input,
        start_ts,
        fmt(args.end),
        dur_ts,
        &args.output
    );

    Ok(())
}

/// Ensure the input file exists
fn existing_file(s: &str) -> Result<String, String> {
    if Path::new(s).is_file() {
        Ok(s.to_string())
    } else {
        Err(format!("input file '{}' does not exist", s))
    }
}

/// Parse HH:MM:SS / MM:SS or human durations like "75s","1m30s"
fn parse_human_time(s: &str) -> Result<Duration, String> {
    if s.contains(':') {
        let parts: Vec<_> = s.split(':').collect();
        let secs = match parts.len() {
            2 => {
                let m = parts[0].parse::<u64>()
                    .map_err(|e| format!("invalid minutes in '{}': {}", s, e))?;
                let s = parts[1].parse::<u64>()
                    .map_err(|e| format!("invalid seconds in '{}': {}", s, e))?;
                m * 60 + s
            }
            3 => {
                let h = parts[0].parse::<u64>()
                    .map_err(|e| format!("invalid hours in '{}': {}", s, e))?;
                let m = parts[1].parse::<u64>()
                    .map_err(|e| format!("invalid minutes in '{}': {}", s, e))?;
                let s = parts[2].parse::<u64>()
                    .map_err(|e| format!("invalid seconds in '{}': {}", s, e))?;
                h * 3600 + m * 60 + s
            }
            _ => return Err(format!("invalid time format '{}'", s)),
        };
        Ok(Duration::from_secs(secs))
    } else {
        s.parse::<HumanDuration>()
            .map(|d| d.into())
            .map_err(|e| format!("invalid time '{}': {}", s, e))
    }
}
