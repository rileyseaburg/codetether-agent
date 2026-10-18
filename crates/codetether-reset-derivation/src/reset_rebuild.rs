//! Rebuild the derived context from a summarised prefix and preserved tail.

use std::sync::Arc;

use anyhow::Result;
use tokio::sync::mpsc;

use codetether_provider_core::Message;
use codetether_session_events::SessionEvent;

use super::reset_summary::summarise_prefix_for_reset;
use codetether_context_policy::projection::helpers::DerivedContext;

/// Summarise `prefix` via RLM, prepend the summary message, append
/// `tail`, repair orphans, and return the resulting [`DerivedContext`].
///
/// `base_index` is the transcript index of `prefix[0]`, used to render
/// the dropped-range TOC with turn numbers that match `context_browse`.
#[allow(clippy::too_many_arguments)]
pub(super) async fn rebuild_with_summary<T: crate::Source, S>(
    session: &T,
    prefix: &[Message],
    tail: &[Message],
    provider: Arc<dyn codetether_provider_core::Provider>,
    model: &str,
    origin_len: usize,
    base_index: usize,
    event_tx: Option<&mpsc::Sender<SessionEvent<S>>>,
    resolver: codetether_rlm_background::Resolver,
) -> Result<DerivedContext> {
    let rlm_config = session.rlm_config().clone();
    let summary = summarise_prefix_for_reset(
        prefix,
        session.session_id(),
        Arc::clone(&provider),
        model,
        &rlm_config,
        session.subcall_provider().cloned(),
        session.subcall_model_name().map(str::to_string),
        event_tx,
        resolver,
    )
    .await?;

    let toc = codetether_context_policy::compression::summary::toc::render_toc(prefix, base_index);

    Ok(codetether_context_policy::derivation::reset::rebuild(
        &summary, &toc, tail, origin_len,
    ))
}
