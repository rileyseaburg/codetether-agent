//! append error classes policy.
use super::*;

pub(super) fn append_error_classes(text: &str, out: &mut Vec<String>) {
    let lower = text.to_lowercase();
    for marker in ERROR_MARKERS {
        if lower.contains(marker) {
            let tag = marker.trim_end_matches(':').to_string();
            if !out.contains(&tag) {
                out.push(tag);
            }
        }
    }
}
