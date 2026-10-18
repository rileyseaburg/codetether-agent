//! Durable Codex-compatible marker for an intentionally interrupted turn.

use crate::MarkerHost;
use anyhow::{Context, Result};
use codetether_provider_core::{ContentPart, Message, Role};

/// Developer message retained after an intentional interruption.
pub const INTERRUPTION_MARKER: &str = "The previous turn was interrupted on purpose. Any running unified exec processes may still be running in the background. If any tools/commands were aborted, they may have partially executed.";

pub(super) async fn persist<H: MarkerHost>(agent_id: &str) -> Result<()> {
    let entry =
        H::get(agent_id).context("interrupted child disappeared before history persistence")?;
    let mut session = entry.session;
    H::add_message(
        &mut session,
        Message {
            role: Role::Developer,
            content: vec![ContentPart::Text {
                text: INTERRUPTION_MARKER.into(),
            }],
        },
    );
    H::save(&session).await?;
    H::update_session(agent_id, session);
    Ok(())
}
