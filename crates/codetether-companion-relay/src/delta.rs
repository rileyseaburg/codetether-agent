//! Delta sink: append bounded analysis text for the current generation only.
use crate::{Delta, Shared, events::event};
use codetether_companion_protocol::EventKind;
use tokio_util::sync::CancellationToken;

/// Build the analyzer callback; overflow past 32,000 characters cancels.
pub(crate) fn sink(relay: Shared, id: String, generation: u64, cancel: CancellationToken) -> Delta {
    Box::new(move |text: &str| {
        let mut state = relay.lock();
        let Some(rt) = state.runtimes.get_mut(&id) else {
            return;
        };
        if cancel.is_cancelled() || rt.stopped || rt.generation != generation {
            return;
        }
        if rt.text.len() + text.len() > 32_000 {
            cancel.cancel();
            return;
        }
        rt.text.push_str(text);
        rt.publish(event(EventKind::Delta, Some(text.to_string()), None));
    })
}
