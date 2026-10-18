//! Select dirty suffixes and encode only the records that need persistence.
use crate::{Batch, Checkpoint, Records, Snapshot, encode};
use anyhow::Result;
/// Encode dirty message/page and tool suffixes for an optimistic commit.
/// # Arguments
/// `s` supplies tracked vectors and a header; `c` is the last checkpoint.
/// # Returns
/// An owned delta with absolute sequence ranges and the expected revision.
/// # Errors
/// Native header or record serialization fails.
/// # Panics
/// Panics if a tool vector violates the invariant that its dirty index is at
/// or before its loaded length.
/// # Examples
/// ```rust,no_run
/// use codetether_session_store::{Snapshot, Checkpoint, prepare};
/// # fn example<S: Snapshot>(session: &S, checkpoint: &Checkpoint) -> anyhow::Result<()> {
/// let delta = prepare(session, checkpoint)?;
/// assert_eq!(delta.revision, checkpoint.revision);
/// # Ok(()) }
/// ```
pub fn prepare<S: Snapshot>(s: &S, c: &Checkpoint) -> Result<Batch> {
    let fresh = c.revision == 0;
    let replaced = s.messages().dirty_from() == usize::MAX
        && s.pages().dirty_from() == usize::MAX
        && [s.messages().version(), s.pages().version()] != c.versions[..2];
    let dirty = if fresh || replaced {
        0
    } else {
        s.messages().dirty_from().min(s.pages().dirty_from())
    };
    let from = dirty.min(s.messages().len());
    let rows = encode::messages(s, from)?;
    let replaced_tools =
        s.tools().dirty_from() == usize::MAX && s.tools().version() != c.versions[2];
    let tool_from = if fresh || replaced_tools {
        0
    } else {
        s.tools().dirty_from().min(s.tools().len())
    };
    let tools = s.tools()[tool_from..]
        .iter()
        .map(serde_json::to_string)
        .collect::<Result<Vec<_>, _>>()?;
    Ok(Batch {
        id: s.id().to_owned(),
        revision: c.revision,
        header: s.encode_header()?,
        messages: Records {
            start: c.message_start + from,
            end: c.message_start + s.messages().len(),
            rows,
        },
        tools: Records {
            start: c.tool_start + tool_from,
            end: c.tool_start + s.tools().len(),
            rows: tools,
        },
    })
}
