//! Router for Ralph loop progress entries.

use crate::bus::BusMessage;

use super::{entry_parts::EntryParts, entry_ralph_parts};

pub(super) fn entry_parts(message: &BusMessage) -> EntryParts {
    if let BusMessage::RalphLearning {
        prd_id,
        story_id,
        iteration,
        learnings,
        ..
    } = message
    {
        return entry_ralph_parts::learning(prd_id, story_id, *iteration, learnings);
    }
    if let BusMessage::RalphHandoff {
        prd_id,
        from_story,
        to_story,
        progress_summary,
        ..
    } = message
    {
        return entry_ralph_parts::handoff(prd_id, from_story, to_story, progress_summary);
    }
    if let BusMessage::RalphProgress {
        prd_id,
        passed,
        total,
        iteration,
        status,
    } = message
    {
        return entry_ralph_parts::progress(prd_id, *passed, *total, *iteration, status);
    }
    unreachable!("ralph entry builder received a non-ralph bus message");
}
