use super::legacy::normalize_line;
use super::summary_count::count_response_item;
use super::summary_text::first_user_text;
use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub(crate) struct CodexSummaryScan {
    pub(crate) updated_at: Option<DateTime<Utc>>,
    pub(crate) message_count: usize,
    pub(crate) first_user_text: Option<String>,
}

pub(crate) fn scan_codex_summary(
    path: &Path,
    session_ts: DateTime<Utc>,
) -> Result<CodexSummaryScan> {
    let file = File::open(path)
        .with_context(|| format!("Failed to open Codex session {}", path.display()))?;
    let mut scan = CodexSummaryScan {
        updated_at: None,
        message_count: 0,
        first_user_text: None,
    };
    for line in BufReader::new(file).lines() {
        let line = line.with_context(|| format!("Failed to read {}", path.display()))?;
        let Some(record) = normalize_line(&line, Some(session_ts))? else {
            continue;
        };
        scan.updated_at = Some(
            scan.updated_at
                .map_or(record.timestamp, |at| at.max(record.timestamp)),
        );
        if record.kind != "response_item" {
            continue;
        }
        if count_response_item(&record.payload) {
            scan.message_count = scan.message_count.saturating_add(1);
        }
        if scan.first_user_text.is_none() {
            scan.first_user_text = first_user_text(&record.payload);
        }
    }
    Ok(scan)
}
