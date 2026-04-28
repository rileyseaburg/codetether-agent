//! Pin/unpin logic for [`ContextPinTool`].

use crate::session::Session;
use crate::session::pages::PageKind;
use anyhow::Result;

/// Apply pin/unpin to session pages and persist.
pub async fn apply_pin(session: &mut Session, idx: usize, action: &str) -> Result<String> {
    if idx >= session.pages.len() {
        anyhow::bail!(
            "turn_index={idx} out of range (len={})",
            session.pages.len()
        );
    }
    match action {
        "pin" => session.pages[idx] = PageKind::Constraint,
        "unpin" => session.pages[idx] = PageKind::Conversation,
        _ => anyhow::bail!("`action` must be \"pin\" or \"unpin\"."),
    }
    session.save().await.map_err(|e| anyhow::anyhow!(e))?;
    Ok(format!("Turn {idx} {action}ned."))
}
