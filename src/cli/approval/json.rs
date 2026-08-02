use super::records::ApprovalRecord;
use crate::approval::ApprovalStatus;
use anyhow::Result;
use serde_json::{Value, json};

/// Serialize records as a JSON array for `approval list --json`.
///
/// # Arguments
///
/// * `records` — Requests with any decision already applied.
///
/// # Returns
///
/// A pretty-printed JSON array. An empty store yields `[]` rather than a message,
/// so a caller can parse the output unconditionally.
///
/// # Errors
///
/// Returns an error when serialization fails.
pub(crate) fn list(records: &[ApprovalRecord]) -> Result<String> {
    let entries: Vec<Value> = records.iter().map(entry).collect();
    Ok(serde_json::to_string_pretty(&entries)?)
}

/// Serialize one record for `approval show --json`.
///
/// # Errors
///
/// Returns an error when serialization fails.
pub(crate) fn show(record: &ApprovalRecord) -> Result<String> {
    Ok(serde_json::to_string_pretty(&entry(record))?)
}

/// Build the JSON object for a single record.
///
/// `status` is emitted explicitly because it is derived from the presence of a
/// decision rather than stored on the request, so a caller cannot compute it.
fn entry(record: &ApprovalRecord) -> Value {
    json!({
        "id": record.request.id,
        "status": status(record.status()),
        "tool": record.request.tool,
        "action": record.request.action,
        "resource": record.request.resource,
        "reason": record.request.reason,
        "requested_at": record.request.requested_at.to_rfc3339(),
        "decision": record.decision.as_ref().map(|decision| {
            json!({
                "id": decision.id,
                "decided_by": decision.decided_by,
                "decided_at": decision.decided_at.to_rfc3339(),
                "reason": decision.reason,
            })
        }),
    })
}

/// Machine-readable status string, matching the text output's spelling.
fn status(status: ApprovalStatus) -> &'static str {
    match status {
        ApprovalStatus::Pending => "pending",
        ApprovalStatus::Approved => "approved",
        ApprovalStatus::Denied => "denied",
    }
}
