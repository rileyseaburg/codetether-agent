//! Rebuild the derived context from a summarised prefix and preserved tail.

use std::sync::Arc;

use anyhow::Result;

use crate::provider::Message;
use crate::session::Session;
use crate::session::helper::experimental;

use super::helpers::DerivedContext;
use super::reset_helpers::build_reset_summary_message;
use super::reset_summary::summarise_prefix_for_reset;

/// Summarise `prefix` via RLM, prepend the summary message, append
/// `tail`, repair orphans, and return the resulting [`DerivedContext`].
pub(super) async fn rebuild_with_summary(
    session: &Session,
    prefix: &[Message],
    tail: &[Message],
    provider: Arc<dyn crate::provider::Provider>,
    model: &str,
    origin_len: usize,
) -> Result<DerivedContext> {
    let rlm_config = session.metadata.rlm.clone();
    let summary = summarise_prefix_for_reset(
        prefix,
        &session.id,
        Arc::clone(&provider),
        model,
        &rlm_config,
        session.metadata.subcall_provider.clone(),
        session.metadata.subcall_model_name.clone(),
    )
    .await?;

    let mut reset_messages = Vec::with_capacity(1 + tail.len());
    reset_messages.push(build_reset_summary_message(&summary));
    reset_messages.extend_from_slice(tail);

    experimental::pairing::repair_orphans(&mut reset_messages);

    Ok(DerivedContext {
        messages: reset_messages,
        origin_len,
        compressed: true,
    })
}
