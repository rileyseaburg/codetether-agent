//! Rebuild the derived context from a summarised prefix and preserved tail.

use std::sync::Arc;

use anyhow::Result;
use tokio::sync::mpsc;

use crate::provider::Message;
use crate::session::Session;
use crate::session::SessionEvent;

use super::helpers::DerivedContext;
use super::reset_summary::summarise_prefix_for_reset;

/// Summarise `prefix` via RLM, prepend the summary message, append
/// `tail`, repair orphans, and return the resulting [`DerivedContext`].
///
/// `base_index` is the transcript index of `prefix[0]`, used to render
/// the dropped-range TOC with turn numbers that match `context_browse`.
#[allow(clippy::too_many_arguments)]
pub(super) async fn rebuild_with_summary(
    session: &Session,
    prefix: &[Message],
    tail: &[Message],
    provider: Arc<dyn crate::provider::Provider>,
    model: &str,
    origin_len: usize,
    base_index: usize,
    event_tx: Option<&mpsc::Sender<SessionEvent>>,
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
        event_tx,
    )
    .await?;

    let toc = crate::session::helper::compression::dropped_toc::render_toc(prefix, base_index);

    Ok(codetether_context_policy::derivation::reset::rebuild(
        &summary, &toc, tail, origin_len,
    ))
}
