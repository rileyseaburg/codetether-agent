//! Pin/unpin logic for [`crate::ContextPinTool`].

use crate::PinSource;
use anyhow::Result;
use codetether_context_policy::pages::PageKind;

/// Apply pin/unpin to session pages and persist.
pub(super) async fn apply_pin<S: PinSource>(
    session: &mut S,
    idx: usize,
    action: &str,
    replacement: Option<usize>,
) -> Result<String> {
    let local = local_index(session, idx)?;
    let previous = session
        .page_kind(local)
        .ok_or_else(|| anyhow::anyhow!("source page is missing"))?;
    let next = replacement.and_then(|absolute| session.source_index(absolute));
    let next_page = next.and_then(|index| session.page_kind(index));
    match action {
        "pin" => session.set_page_kind(local, PageKind::Constraint),
        "unpin" => {
            if !matches!(
                session.page_kind(local),
                Some(PageKind::SupersededConstraint { .. })
            ) {
                session.set_page_kind(local, PageKind::Conversation);
            }
        }
        "supersede" => super::supersede::apply(session, idx, local, replacement)?,
        _ => anyhow::bail!("`action` must be pin, unpin, or supersede."),
    }
    if let Err(error) = session.persist().await {
        session.set_page_kind(local, previous);
        if let (true, Some((next, page))) = (action == "supersede", next.zip(next_page)) {
            session.set_page_kind(next, page);
        }
        return Err(error);
    }
    Ok(match action {
        "supersede" => format!(
            "Turn {idx} superseded by user source turn {}.",
            replacement.unwrap()
        ),
        "pin" => format!("Turn {idx} pinned."),
        "unpin" => format!("Turn {idx} unpinned."),
        _ => unreachable!("action was validated before persistence"),
    })
}

pub(super) fn local_index<S: PinSource>(session: &S, absolute: usize) -> Result<usize> {
    let local = session.source_index(absolute);
    local.ok_or_else(|| {
        anyhow::anyhow!(
            "turn_index={absolute} is outside the loaded absolute range {}..{}",
            session.page_offset(),
            session.page_offset().saturating_add(session.page_count())
        )
    })
}
