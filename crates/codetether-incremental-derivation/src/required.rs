//! Protect base pins and the active user instruction without pinning all tool history.
use codetether_context_policy::pages::PageKind;
use codetether_provider_core::Message;

pub fn seed(
    messages: &[Message],
    pages: &[PageKind],
    keep: &mut [bool],
    costs: &[usize],
    recent_start: usize,
    budget: &mut usize,
) {
    codetether_context_policy::pins::force_keep_base(
        pages,
        messages,
        keep,
        costs,
        recent_start,
        budget,
    );
    if let Some(anchor) =
        codetether_context_policy::selection::active_tail::active_user_tail_start(messages, 0)
        && !keep[anchor]
    {
        keep[anchor] = true;
        *budget = budget.saturating_sub(costs[anchor]);
    }
}

pub(super) fn seed_source<H: crate::Source>(
    session: &H,
    keep: &mut [bool],
    costs: &[usize],
    recent_start: usize,
    budget: &mut usize,
) {
    seed(
        session.messages(),
        session.pages(),
        keep,
        costs,
        recent_start,
        budget,
    );
}
