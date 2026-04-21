//! [`DerivePolicy::Reset`] — Lu et al. reset-to-(prompt, summary) semantic.

use std::sync::Arc;

use anyhow::Result;

use crate::provider::ToolDefinition;
use crate::session::Session;
use crate::session::helper::experimental;
use crate::session::helper::token::estimate_request_tokens;

use super::derive::derive_context;
use super::helpers::DerivedContext;
use super::reset_helpers::last_user_index;
use super::reset_rebuild::rebuild_with_summary;

/// [`DerivePolicy::Reset`](crate::session::derive_policy::DerivePolicy::Reset) implementation.
///
/// When the token estimate exceeds `threshold_tokens`, summarise
/// everything older than the last user turn via the RLM router and
/// return `[summary_message, last_user_turn]`. Under threshold, return
/// the full clone verbatim.
pub(super) async fn derive_reset(
    session: &Session,
    provider: Arc<dyn crate::provider::Provider>,
    model: &str,
    system_prompt: &str,
    tools: &[ToolDefinition],
    threshold_tokens: usize,
) -> Result<DerivedContext> {
    let origin_len = session.messages.len();
    let mut messages = session.messages.clone();

    let est = estimate_request_tokens(system_prompt, &messages, tools);
    if est <= threshold_tokens {
        experimental::pairing::repair_orphans(&mut messages);
        return Ok(DerivedContext {
            messages,
            origin_len,
            compressed: false,
        });
    }

    let Some(split_idx) = last_user_index(&messages) else {
        return derive_context(session, provider, model, system_prompt, tools, None, None).await;
    };

    let tail = messages.split_off(split_idx);
    let prefix = std::mem::take(&mut messages);
    if prefix.is_empty() {
        messages = tail;
        experimental::pairing::repair_orphans(&mut messages);
        return Ok(DerivedContext {
            messages,
            origin_len,
            compressed: false,
        });
    }

    rebuild_with_summary(session, &prefix, &tail, provider, model, origin_len).await
}
