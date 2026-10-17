//! Pin/unpin logic for [`crate::ContextPinTool`].

use crate::PinSource;
use anyhow::Result;
use codetether_context_policy::pages::PageKind;

/// Apply pin/unpin to session pages and persist.
pub(super) async fn apply_pin<S: PinSource>(
    session: &mut S,
    idx: usize,
    action: &str,
) -> Result<String> {
    if idx >= session.page_count() {
        anyhow::bail!(
            "turn_index={idx} out of range (len={})",
            session.page_count()
        );
    }
    match action {
        "pin" => session.set_page_kind(idx, PageKind::Constraint),
        "unpin" => session.set_page_kind(idx, PageKind::Conversation),
        _ => anyhow::bail!("`action` must be \"pin\" or \"unpin\"."),
    }
    session.persist().await.map_err(|e| anyhow::anyhow!(e))?;
    Ok(format!("Turn {idx} {action}ned."))
}
