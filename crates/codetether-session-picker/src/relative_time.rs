//! Human-friendly relative timestamp rendering for session rows.

use chrono::{DateTime, Utc};

/// Render `ts` as a compact age relative to now (e.g. `3m`, `2h`, `5d`).
///
/// Falls back to an absolute `YYYY-MM-DD` date once older than ~30 days
/// so long-lived sessions stay unambiguous.
pub fn relative_time(ts: DateTime<Utc>) -> String {
    let secs = (Utc::now() - ts).num_seconds().max(0);
    match secs {
        s if s < 60 => "just now".to_string(),
        s if s < 3_600 => format!("{}m ago", s / 60),
        s if s < 86_400 => format!("{}h ago", s / 3_600),
        s if s < 2_592_000 => format!("{}d ago", s / 86_400),
        _ => ts.format("%Y-%m-%d").to_string(),
    }
}
