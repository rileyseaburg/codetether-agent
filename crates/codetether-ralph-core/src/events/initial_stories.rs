//! Build initial story presentation data from PRD progress.
use super::{RalphStoryInfo, RalphStoryStatus};
use crate::Prd;
/// Project PRD stories to their initial UI state in PRD order.
///
/// # Arguments
/// `prd` supplies story identity, dependencies, priority and completion.
/// # Returns
/// Story summaries with empty live activity fields.
/// # Examples
/// ```
/// let prd = codetether_ralph_core::create_prd_template("example", "feature");
/// let stories = codetether_ralph_core::events::initial_stories(&prd);
/// assert_eq!(stories.len(), prd.user_stories.len());
/// ```

/// Build initial RalphStoryInfo list from the PRD
pub fn initial_stories(prd: &Prd) -> Vec<RalphStoryInfo> {
    prd.user_stories
        .iter()
        .map(|s| RalphStoryInfo {
            id: s.id.clone(),
            title: s.title.clone(),
            status: if s.passes {
                RalphStoryStatus::Passed
            } else {
                RalphStoryStatus::Pending
            },
            priority: s.priority,
            depends_on: s.depends_on.clone(),
            quality_checks: Vec::new(),
            tool_call_history: Vec::new(),
            messages: Vec::new(),
            output: None,
            error: None,
            merge_summary: None,
            steps: 0,
            current_tool: None,
        })
        .collect()
}
