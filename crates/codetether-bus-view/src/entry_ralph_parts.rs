//! Ralph loop entry payload builders.

use ratatui::style::Color;

use super::entry_parts::EntryParts;

pub(super) fn learning(
    prd_id: &str,
    story_id: &str,
    iteration: usize,
    learnings: &[String],
) -> EntryParts {
    EntryParts::new(
        "LEARN",
        format!("{story_id} iter {iteration} ({} items)", learnings.len()),
        format!(
            "PRD: {prd_id}\nStory: {story_id}\nIteration: {iteration}\nLearnings:\n{}",
            learnings.join("\n")
        ),
        Color::Cyan,
    )
}

pub(super) fn handoff(prd_id: &str, from_story: &str, to_story: &str, summary: &str) -> EntryParts {
    EntryParts::new(
        "HANDOFF",
        format!("{from_story} → {to_story}"),
        format!("PRD: {prd_id}\nFrom: {from_story}\nTo: {to_story}\nSummary: {summary}"),
        Color::Blue,
    )
}

pub(super) fn progress(
    prd_id: &str,
    passed: usize,
    total: usize,
    iteration: usize,
    status: &str,
) -> EntryParts {
    EntryParts::new(
        "PRD",
        format!("{passed}/{total} stories (iter {iteration}) [{status}]"),
        format!(
            "PRD: {prd_id}\nPassed: {passed}/{total}\nIteration: {iteration}\nStatus: {status}"
        ),
        Color::Yellow,
    )
}
