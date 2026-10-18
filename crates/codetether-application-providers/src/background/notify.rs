//! Convert worker progress into the caller's typed session event stream.
use super::Notify;
use codetether_session_events::SessionEvent;
use std::sync::Arc;

pub(super) fn adapt<S: 'static>(
    notify: Option<Notify<S>>,
) -> Option<codetether_rlm_background::Notify> {
    notify.map(|notify| {
        Arc::new(move |progress| notify(SessionEvent::RlmProgress(progress)))
            as codetether_rlm_background::Notify
    })
}
