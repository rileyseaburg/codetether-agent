//! Validate both absolute sources before atomically staging a supersession.
use crate::{PinSource, logic::local_index};
use anyhow::{Result, ensure};
use codetether_context_policy::pages::PageKind;
use codetether_provider_core::Role;

pub(super) fn apply<S: PinSource>(
    session: &mut S,
    old: usize,
    local: usize,
    replacement: Option<usize>,
) -> Result<()> {
    let replacement =
        replacement.ok_or_else(|| anyhow::anyhow!("supersede requires replacement_index"))?;
    ensure!(
        replacement > old,
        "a replacement must be a later source turn"
    );
    let next = local_index(session, replacement)?;
    let already_replaced = session.page_kind(local)
        == Some(PageKind::SupersededConstraint {
            by_turn: replacement,
        });
    ensure!(
        !matches!(
            session.page_kind(next),
            Some(PageKind::SupersededConstraint { .. })
        ),
        "replacement source has itself been superseded; use the current correction"
    );
    ensure!(
        !already_replaced || session.page_kind(next) == Some(PageKind::Constraint),
        "replacement pin changed after supersession; a stale retry cannot reactivate it"
    );
    ensure!(
        matches!(session.page_kind(local), Some(PageKind::Constraint)) || already_replaced,
        "the replaced turn must be an active pin or already point to this replacement"
    );
    let message = session
        .source_message(next)
        .ok_or_else(|| anyhow::anyhow!("replacement source is missing"))?;
    ensure!(
        message.role == Role::User,
        "a replacement constraint must reference a user message, not an assistant inference or tool output"
    );
    ensure!(
        codetether_context_policy::state_header::ConstraintRecord::from_message(
            replacement,
            &message,
            PageKind::Constraint
        )
        .is_some(),
        "replacement source has no text"
    );
    session.set_page_kind(
        local,
        PageKind::SupersededConstraint {
            by_turn: replacement,
        },
    );
    session.set_page_kind(next, PageKind::Constraint);
    Ok(())
}
