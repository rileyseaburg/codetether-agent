//! Validate chronological per-call pairing without requiring role-contiguous batches.
use super::{TrainingRecord, is_request, is_response};
use std::collections::BTreeSet;

pub(super) fn complete(records: &[TrainingRecord]) -> Result<(), &'static str> {
    let mut seen = BTreeSet::new();
    let mut pending = BTreeSet::new();
    for record in records {
        if is_request(record) {
            let id = record.tool_calls.as_ref().unwrap()[0].id.as_str();
            if id.is_empty() {
                return Err("empty_request_id");
            }
            if !seen.insert(id) {
                return Err("duplicate_request");
            }
            pending.insert(id);
        } else if is_response(record) {
            let id = record.tool_call_id.as_deref().unwrap();
            if !seen.contains(id) {
                return Err("response_without_prior_request");
            }
            if !pending.remove(id) {
                return Err("duplicate_response");
            }
        } else {
            return Err("invalid_tool_record");
        }
    }
    if seen.is_empty() {
        Err("missing_requests")
    } else if !pending.is_empty() {
        Err("missing_responses")
    } else {
        Ok(())
    }
}
