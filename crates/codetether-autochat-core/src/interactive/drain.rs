//! Drain queued relay events without retaining the receiver borrow.
use super::host::View;

/// Collect pending relay events, then apply them in arrival order.
///
/// # Arguments
/// * `app` — Relay state and event presentation callbacks.
///
/// # Returns
/// Unit after applying every currently queued event. The receiver is borrowed
/// only during collection, allowing handlers to mutate the same view safely.
///
/// # Examples
/// ```no_run
/// fn tick<E>(view: &mut impl codetether_autochat_core::interactive::host::View<E>) {
///     codetether_autochat_core::interactive::drain::drain_autochat(view);
/// }
/// ```
pub fn drain_autochat<E>(app: &mut impl View<E>) {
    let events: Vec<_> = app
        .autochat()
        .rx
        .as_mut()
        .map(|rx| {
            let mut evts = Vec::new();
            while let Ok(e) = rx.try_recv() {
                evts.push(e);
            }
            evts
        })
        .unwrap_or_default();
    for event in events {
        super::handler::handle_autochat_event(app, event);
    }
}
