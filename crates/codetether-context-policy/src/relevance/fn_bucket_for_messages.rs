//! bucket for messages policy.
use super::*;

/// Project a coarse CADMAS-CTX bucket from a recent message window.
///
/// Merges the last few entries into a single synthetic
/// [`RelevanceMeta`] so routing decisions can key off the active task
/// rather than a single arbitrary turn.
pub fn bucket_for_messages(messages: &[Message]) -> Bucket {
    let start = messages.len().saturating_sub(8);
    let mut merged = RelevanceMeta::default();
    for msg in &messages[start..] {
        let next = extract(msg);
        merged.files.extend(next.files);
        merged.tools.extend(next.tools);
        merged.error_classes.extend(next.error_classes);
        merged.explicit_refs.extend(next.explicit_refs);
    }
    dedupe_preserving_order(&mut merged.files);
    dedupe_preserving_order(&mut merged.tools);
    dedupe_preserving_order(&mut merged.error_classes);
    merged.project_bucket()
}
