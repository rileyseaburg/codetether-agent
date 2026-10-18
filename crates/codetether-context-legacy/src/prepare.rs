//! Clone source history, bound its final message and apply native transforms.
use crate::{Hooks, Source};
use anyhow::Result;
use codetether_context_compression::{CompressContext, compress_last_message_if_oversized};
use codetether_provider_core::{Message, Provider};
use std::sync::Arc;
pub(super) struct Prepared {
    pub origin_len: usize,
    pub messages: Vec<Message>,
    pub ctx: CompressContext,
    pub step0: bool,
    pub before: usize,
}
pub(super) async fn prepare<T: Source>(
    session: &T,
    provider: &Arc<dyn Provider>,
    model: &str,
    hooks: &Hooks,
) -> Result<Prepared> {
    let origin_len = session.messages().len();
    let mut messages = session.messages().to_vec();
    let ctx = CompressContext::new(session.session_id(), session.rlm_config());

    let step0 = compress_last_message_if_oversized(
        &mut messages,
        &ctx,
        Arc::clone(provider),
        model,
        hooks.resolver,
    )
    .await?;

    (hooks.apply)(&mut messages);

    let before = messages.len();

    Ok(Prepared {
        origin_len,
        messages,
        ctx,
        step0,
        before,
    })
}
