//! Relay context merge bucket.
use super::*;

pub(super) fn merge_bucket(target: &mut Vec<String>, additions: &[String]) {
    for item in additions {
        let normalized = item.trim();
        if normalized.is_empty() {
            continue;
        }
        if target.iter().any(|existing| existing == normalized) {
            continue;
        }
        target.push(normalized.to_string());
        if target.len() > CONTEXT_BUCKET_LIMIT {
            target.remove(0);
        }
    }
}
