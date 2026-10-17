//! Render per-story evidence for every objective key result.

use codetether_ralph_core::Prd;
pub(crate) fn collect(prd: &Prd) -> Vec<String> {
    prd.user_stories
        .iter()
        .map(|s| {
            format!(
                "{}:{} ({})",
                s.id,
                s.title,
                if s.passes { "PASSED" } else { "FAILED" }
            )
        })
        .collect()
}
