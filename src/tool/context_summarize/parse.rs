//! Argument parsing for `context_summarize`.

use crate::session::index::types::SummaryRange;

/// Parsed summarize request.
pub struct SummaryArgs {
    pub range: SummaryRange,
    pub target: usize,
}

/// Parse and validate tool arguments.
pub fn parse(args: &serde_json::Value) -> Result<SummaryArgs, &'static str> {
    let start = args["start"].as_u64().unwrap_or(0) as usize;
    let end = args["end"].as_u64().unwrap_or(0) as usize;
    let target = args["target_tokens"].as_u64().unwrap_or(512) as usize;
    let range = SummaryRange::new(start, end).ok_or("Invalid range: start must be < end.")?;
    Ok(SummaryArgs { range, target })
}
