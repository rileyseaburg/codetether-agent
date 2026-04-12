//! Autochat relay drain on the background tick.
//!
//! Collects pending autochat events from the channel and
//! dispatches them without holding a mutable borrow across
//! the entire receive loop.
//!
//! # Examples
//!
//! ```ignore
//! drain_autochat(&mut app);
//! ```

use crate::tui::app::state::App;

/// Drain all pending autochat relay events.
///
/// Collects events into a `Vec` first to avoid holding two
/// mutable borrows on `app.state` simultaneously.
///
/// # Examples
///
/// ```ignore
/// drain_autochat(&mut app);
/// ```
pub(super) fn drain_autochat(app: &mut App) {
    let events: Vec<_> = app
        .state
        .autochat
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
        super::super::autochat::handle_autochat_event(&mut app.state, event);
    }
}
