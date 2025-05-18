use humantime::Duration as HumanDuration;
use std::{time::Duration, num::ParseIntError};

/// Parse “75s”, “1m30s”, “MM:SS” or “HH:MM:SS”
pub fn parse_time(s: &str) -> Result<Duration, String> {
    if s.contains(':') {
        let parts: Vec<_> = s.split(':').collect();
        let to_secs = |p: &str, name: &str| -> Result<u64, String> {
            p.parse::<u64>()
                .map_err(|e| format!("invalid {} in '{}': {}", name, s, e))
        };
        let secs = match parts.len() {
            2 => {
                let m = to_secs(parts[0], "minutes")?;
                let s = to_secs(parts[1], "seconds")?;
                m * 60 + s
            }
            3 => {
                let h = to_secs(parts[0], "hours")?;
                let m = to_secs(parts[1], "minutes")?;
                let s = to_secs(parts[2], "seconds")?;
                h * 3600 + m * 60 + s
            }
            _ => return Err(format!("invalid time format '{}'", s)),
        };
        Ok(Duration::from_secs(secs))
    } else {
        s.parse::<HumanDuration>()
            .map(|d| d.into())
            .map_err(|e| format!("invalid duration '{}': {}", s, e))
    }
}

pub fn validate_range(start: Duration, end: Duration) -> Result<(), String> {
    if start >= end {
        Err(format!(
            "start ({:?}) must be less than end ({:?})",
            start, end
        ))
    } else {
        Ok(())
    }
}
