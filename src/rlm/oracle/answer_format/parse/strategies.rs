//! Per-format parsing strategies for FINAL() answers.
//!
//! Each function attempts to parse the raw answer into a single
//! [`FinalAnswerFormat`] variant, returning `None` on mismatch.
//!
//! # Examples
//!
//! ```ignore
//! if let Some(fmt) = try_parse_line_numbered("42:fn foo()") { .. }
//! ```

use super::super::FinalAnswerFormat;
use super::extract_count_from_text;

pub fn try_parse_line_numbered(answer: &str) -> Option<FinalAnswerFormat> {
    let mut matches = Vec::new();
    for line in answer.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        let colon_pos = trimmed.find(':')?;
        let num_part = trimmed[..colon_pos].trim().trim_start_matches('L').trim();
        let line_num: usize = num_part.parse().ok()?;
        matches.push((line_num, trimmed[colon_pos + 1..].trim().to_string()));
    }
    if matches.is_empty() {
        return None;
    }
    Some(FinalAnswerFormat::LineNumberedMatches { matches })
}

pub fn try_parse_count(answer: &str) -> Option<FinalAnswerFormat> {
    let lower = answer.to_lowercase();
    if !(lower.contains("found") || lower.contains("count:") || lower.contains("occurrences")) {
        return None;
    }
    extract_count_from_text(answer).map(|count| FinalAnswerFormat::CountResult { count })
}

pub fn try_parse_json(answer: &str) -> Option<FinalAnswerFormat> {
    let trimmed = answer.trim();
    if !(trimmed.starts_with('{') || trimmed.starts_with('[')) {
        return None;
    }
    serde_json::from_str(trimmed)
        .ok()
        .map(|data| FinalAnswerFormat::StructuredData { data })
}
