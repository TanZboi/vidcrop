use std::{
    path::Path,
    process::Command,
    time::Duration,
};
use anyhow::{Context, Result};

/// Given “demo.mp4” → “democlip.mp4”
pub fn derive_output(input: &str) -> String {
    let p = Path::new(input);
    let stem = p.file_stem().unwrap().to_string_lossy();
    let ext  = p.extension().and_then(|e| e.to_str()).unwrap_or("mp4");
    format!("{}clip.{}", stem, ext)
}

/// Run ffmpeg with re-encode (frame-accurate)
pub fn run(
    input: &str,
    start: Duration,
    end: Duration,
    output: &str,
) -> Result<()> {
    let fmt = |d: Duration| {
        let secs = d.as_secs();
        let h = secs / 3600;
        let m = (secs % 3600) / 60;
        let s = secs % 60;
        format!("{:02}:{:02}:{:02}", h, m, s)
    };

    let start_ts = fmt(start);
    let dur_ts   = fmt(end - start);

    let ext = Path::new(output)
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("mp4");

    let (vcodec, acodec) = match ext.to_lowercase().as_str() {
        "avi" => ("mpeg4", "mp3"),
        "mov" => ("libx264", "aac"),
        _     => ("libx264", "aac"),
    };

    let status = Command::new("ffmpeg")
        .args(&[
            "-ss",  &start_ts,
            "-i",   input,
            "-t",   &dur_ts,
            "-c:v", vcodec,
            "-c:a", acodec,
            "-y",
            output,
        ])
        .status()
        .context("failed to spawn ffmpeg")?;

    if !status.success() {
        anyhow::bail!("ffmpeg exited: {}", status);
    }
    Ok(())
}
