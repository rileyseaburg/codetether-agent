//! Replace the visible transcript and initialize its pagination boundary.
use super::View;
use codetether_provider_core::Message;
/// Synchronize a loaded transcript with its source history and UI.
///
/// # Arguments
/// * `app` — Presentation and metric bindings.
/// * `history` — Loaded provider messages in chronological order.
/// * `message_count` — Session message count used for the absolute page offset.
/// * `source_id` — Storage identity for loading older messages.
/// * `source_has_older` — Whether older messages were omitted by the loader.
///
/// # Returns
/// Replaces the visible transcript and resets its pagination, cache and metrics.
///
/// # Examples
/// ```no_run
/// # fn sync(view: &mut impl codetether_chat_transcript::sync::View) {
/// codetether_chat_transcript::sync::sync_messages_from_source(view, &[], 0, "session", false);
/// # }
/// ```
pub fn sync_messages_from_source(
    app: &mut impl View,
    history: &[Message],
    message_count: usize,
    source_id: &str,
    source_has_older: bool,
) {
    let visible: Vec<_> = super::recent(history).collect();
    let boundary: Vec<Message> = visible.iter().take(3).map(|item| (*item).clone()).collect();
    let has_older = source_has_older || message_count > visible.len();
    app.replace_messages(crate::provider_messages_to_chat_messages(
        visible.iter().copied(),
    ));
    app.reset_history(source_id.to_string(), &boundary, visible.len(), has_older);
    app.set_history_before(message_count.saturating_sub(visible.len()));
    app.clear_cache();
    app.metrics().reset();
    app.reset_tool_preview_scroll();
    app.set_tool_preview_max_scroll(0);
    app.scroll_to_bottom();
}
