//! Assemble selected summaries and raw messages with aligned origin metadata.
use super::*;
pub(super) fn assemble(
    plan: PreparedIncremental,
    index: &SummaryIndex,
    estimate_messages: impl Fn(&[Message]) -> usize,
) -> ProjectedIncremental {
    let PreparedIncremental {
        clone,
        origin_len,
        budget_tokens,
        mut keep,
        per_msg,
        order,
        budget_for_messages,
    } = plan;
    let gaps = pack::select(
        index,
        &mut keep,
        &per_msg,
        order,
        budget_for_messages,
        estimate_messages,
    );
    let dropped_ranges = collect_dropped_ranges(&keep);
    let (mut messages, _, mut origins) = interleave(&clone, &keep, &gaps);
    repair_with_origins(&mut messages, &mut origins);
    let resolutions: Vec<ResidencyLevel> = messages.iter().map(residency_for_message).collect();

    ProjectedIncremental {
        messages,
        origins,
        resolutions,
        dropped_ranges,
        clone_len: clone.len(),
        origin_len,
        budget_tokens,
    }
}
