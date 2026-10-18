//! Original tool arguments with a fallback for legacy patch previews.
use codetether_approval_queue::ApprovalSnapshot;
use serde_json::{Value, json};
/// Recover tool arguments from a pending request snapshot.
/// # Arguments
/// * `item` - Snapshot with optional original arguments and preview text.
/// # Returns
/// Original JSON when present, a patch argument for legacy patch requests,
/// or None when the request has neither representation.
/// # Examples
/// ```
/// # fn example(item: &codetether_approval_queue::ApprovalSnapshot) {
/// let _ = codetether_approval_diagnostics::arguments(item);
/// # }
/// ```

/// Raw arguments, or a patch-only shape for requests from older runtimes.
pub fn arguments(item: &ApprovalSnapshot) -> Option<Value> {
    if let Some(arguments) = &item.arguments {
        return Some(arguments.clone());
    }
    let legacy_patch = matches!(item.tool.as_str(), "apply_patch" | "patch");
    legacy_patch
        .then(|| item.preview.as_ref())
        .flatten()
        .map(|patch| json!({"patch": patch}))
}
