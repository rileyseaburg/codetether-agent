//! One bounded analysis task, owned independently of viewer connectivity.
use crate::{Analysis, Shared, analysis_finish::finish, capture::Admitted, delta, relay::now};
use std::time::Duration;

/// Spawn the analyzer for an admitted frame with a 120-second limit.
pub(crate) fn spawn(relay: Shared, id: String, image: String, work: Admitted) {
    let Admitted {
        generation,
        cancel,
        prompt,
        type_requested,
    } = work;
    let (model, previous) = {
        let mut state = relay.lock();
        match state.live(&id, now()) {
            Ok(rt) => (rt.model.clone(), rt.previous.clone()),
            Err(_) => return,
        }
    };
    let delta = delta::sink(relay.clone(), id.clone(), generation, cancel.clone());
    let input = Analysis {
        image,
        model,
        prompt,
        previous,
        cancel: cancel.clone(),
        delta,
    };
    let run = (relay.analyze)(input);
    tokio::spawn(async move {
        let outcome = tokio::select! {
            result = tokio::time::timeout(Duration::from_secs(120), run) => result.ok(),
            () = cancel.cancelled() => None,
        };
        if outcome.is_none() {
            cancel.cancel();
        }
        let ok = matches!(outcome, Some(Ok(())));
        finish(&relay, &id, generation, &cancel, ok, type_requested);
    });
}
