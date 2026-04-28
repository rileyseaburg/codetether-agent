//! Pin/unpin logic for [`ContextPinTool`].

use crate::session::Session;
use crate::session::pages::{PageKind, classify};
use anyhow::Result;
use serde_json::Value;

/// Parse `turn_index` from tool args.
pub fn parse_turn_index(args: &Value) -> Result<usize, String> {
    args["turn_index"]
        .as_u64()
        .map(|i| i as usize)
        .ok_or_else(|| "`turn_index` must be a non-negative integer.".to_string())
}

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
        "unpin" => session.pages[idx] = classify(&session.messages[idx]),
        _ => anyhow::bail!("`action` must be \"pin\" or \"unpin\"."),
    }
    session.save().await.map_err(|e| anyhow::anyhow!(e))?;
    Ok(format!("Turn {idx} {action}ned."))
}
