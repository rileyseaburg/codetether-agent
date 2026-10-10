//! Final `done`/`error` publication for an analysis run.
use crate::{Relay, events::event};
use codetether_companion_protocol::EventKind;
use tokio_util::sync::CancellationToken;

const FAILED: &str = "Analysis failed. Check the selected vision model and try again.";
const INTERRUPTED: &str = "Analysis interrupted or exceeded its limit. Request another capture.";

/// Publish the result and release the active slot if still owned.
pub(crate) fn finish(
    relay: &Relay,
    id: &str,
    generation: u64,
    cancel: &CancellationToken,
    ok: bool,
    type_requested: bool,
) {
    let mut state = relay.lock();
    let Ok(rt) = state.live(id, crate::relay::now()) else {
        return;
    };
    let owned = rt.active.as_ref().is_some_and(|(g, _)| *g == generation);
    if !owned || rt.stopped {
        return;
    }
    rt.active = None;
    if cancel.is_cancelled() {
        if rt.pending.is_none() && rt.status != "paused" {
            rt.status = "error".into();
            rt.publish(event(
                EventKind::Error,
                Some(INTERRUPTED.into()),
                Some("error"),
            ));
        }
        return;
    }
    if ok && !rt.text.trim().is_empty() {
        crate::model_typing::finish(rt, type_requested);
        let start = rt.text.len().saturating_sub(4000);
        let start = (start..=rt.text.len())
            .find(|i| rt.text.is_char_boundary(*i))
            .unwrap_or(0);
        rt.previous = rt.text[start..].to_string();
        rt.status = "ready".into();
        let text = rt.text.clone();
        rt.publish(event(EventKind::Done, Some(text), Some("ready")));
    } else {
        tracing::warn!(event = "screen_analysis_failed", model = %rt.model, "Analysis failed");
        rt.status = "error".into();
        rt.publish(event(EventKind::Error, Some(FAILED.into()), Some("error")));
    }
}
