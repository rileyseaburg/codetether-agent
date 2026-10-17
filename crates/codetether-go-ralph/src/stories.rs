//! Collect completed story identity, title and pass status.

use crate::StoryResult;
use codetether_ralph_core::RalphState;
pub(crate) fn collect(state: &RalphState) -> Vec<StoryResult> {
    state
        .prd
        .user_stories
        .iter()
        .map(|s| StoryResult {
            id: s.id.clone(),
            title: s.title.clone(),
            passed: s.passes,
        })
        .collect()
}
