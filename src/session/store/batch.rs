//! Encode only explicitly dirty suffixes, never the retained prefix.
use super::state::Checkpoint;
use crate::session::Session;
use anyhow::Result;
mod encode;
mod types;
pub(super) use types::{Batch, Records};
pub(super) fn prepare(s: &Session, c: &Checkpoint) -> Result<Batch> {
    let fresh = c.revision == 0;
    let replaced = s.messages.dirty_from() == usize::MAX
        && s.pages.dirty_from() == usize::MAX
        && [s.messages.version(), s.pages.version()] != c.versions[..2];
    let dirty = if fresh || replaced {
        0
    } else {
        s.messages.dirty_from().min(s.pages.dirty_from())
    };
    let from = dirty.min(s.messages.len());
    let rows = encode::messages(s, from)?;
    let replaced_tools =
        s.tool_uses.dirty_from() == usize::MAX && s.tool_uses.version() != c.versions[2];
    let tool_from = if fresh || replaced_tools {
        0
    } else {
        s.tool_uses.dirty_from().min(s.tool_uses.len())
    };
    let tools = s.tool_uses[tool_from..]
        .iter()
        .map(serde_json::to_string)
        .collect::<Result<Vec<_>, _>>()?;
    Ok(Batch {
        id: s.id.clone(),
        revision: c.revision,
        header: super::header::encode(s)?,
        messages: Records {
            start: c.message_start + from,
            end: c.message_start + s.messages.len(),
            rows,
        },
        tools: Records {
            start: c.tool_start + tool_from,
            end: c.tool_start + s.tool_uses.len(),
            rows: tools,
        },
    })
}
