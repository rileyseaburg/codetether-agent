//! Deterministic grep answers.

use anyhow::Result;

use crate::oracle::{FinalPayload, GrepMatch, GrepOracle, GrepPayload};

/// Answer pattern queries directly from grep evidence.
pub fn answer(content: &str, query: &str, file: String) -> Result<Option<String>> {
    let Some(pattern) = GrepOracle::infer_pattern(query) else {
        return Ok(None);
    };
    let matches = GrepOracle::new(content.to_string()).grep(&pattern)?;
    let payload = FinalPayload::Grep(GrepPayload {
        file,
        pattern,
        matches: matches
            .into_iter()
            .map(|(line, text)| GrepMatch { line, text })
            .collect(),
    });
    Ok(Some(serde_json::to_string(&payload)?))
}
