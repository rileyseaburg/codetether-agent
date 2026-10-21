//! Strict model-output parsing and exact snapshot replacement checks.
use crate::{Proposal, RepairRequest, paths, validate_request};
use anyhow::{Result, ensure};
use std::collections::HashSet;

/// Parse raw JSON or a single JSON code fence, then validate every edit.
/// # Arguments
/// * `text` - Model response text.
/// * `request` - Original immutable snapshot.
/// # Returns
/// A reviewable proposal; never applies edits.
/// # Errors
/// Rejects malformed envelopes, extra text and invalid replacements.
/// # Examples
/// ```
/// let r = codetether_rust_repair::RepairRequest { schema_version:1, diagnostics:vec![], files:vec![] };
/// assert!(codetether_rust_repair::parse_proposal(r#"{"schema_version":1,"summary":"No safe fix","edits":[]}"#, &r).is_ok());
/// ```
pub fn parse_proposal(text: &str, request: &RepairRequest) -> Result<Proposal> {
    ensure!(
        text.len() <= crate::MAX_REQUEST_BYTES,
        "proposal exceeds 512 KiB"
    );
    let text = text.trim();
    let body = ["```json\n", "```json\r\n", "```\n", "```\r\n"]
        .iter()
        .find_map(|prefix| {
            text.strip_prefix(prefix)
                .and_then(|s| s.strip_suffix("```"))
        })
        .unwrap_or(text);
    let proposal = serde_json::from_str(body)?;
    validate_proposal(request, &proposal)?;
    Ok(proposal)
}

/// Check a proposal against a validated request without filesystem access.
/// # Arguments
/// `request` supplies the snapshots; `proposal` supplies exact replacements.
/// # Returns
/// Unit only for bounded, unique, authorized, non-noop replacements.
/// # Errors
/// Rejects unknown/read-only/protected paths, ambiguous old text, and duplicates.
/// # Examples
/// ```
/// let r = codetether_rust_repair::RepairRequest { schema_version:1, diagnostics:vec![], files:vec![] };
/// let p = codetether_rust_repair::Proposal { schema_version:1, summary:"No safe fix".into(), edits:vec![] };
/// assert!(codetether_rust_repair::validate_proposal(&r, &p).is_ok());
/// ```
pub fn validate_proposal(request: &RepairRequest, proposal: &Proposal) -> Result<()> {
    validate_request(request)?;
    ensure!(
        proposal.schema_version == 1 && !proposal.summary.trim().is_empty(),
        "invalid proposal header"
    );
    ensure!(proposal.edits.len() <= 8, "proposal exceeds eight edits");
    ensure!(
        serde_json::to_vec(proposal)?.len() <= crate::MAX_REQUEST_BYTES,
        "proposal exceeds 512 KiB"
    );
    let mut seen = HashSet::new();
    for edit in &proposal.edits {
        paths::validate_path(&edit.path)?;
        ensure!(
            !paths::forbidden(&edit.path) && seen.insert(&edit.path),
            "protected or repeated edit path: {}",
            edit.path
        );
        let file = request
            .files
            .iter()
            .find(|file| file.path == edit.path && file.editable)
            .ok_or_else(|| anyhow::anyhow!("edit path is not editable: {}", edit.path))?;
        ensure!(
            !edit.old_text.is_empty() && edit.old_text != edit.new_text,
            "empty or unchanged replacement"
        );
        let first = file.text.find(&edit.old_text);
        ensure!(
            first.is_some() && first == file.text.rfind(&edit.old_text),
            "old_text must occur exactly once: {}",
            edit.path
        );
    }
    Ok(())
}
