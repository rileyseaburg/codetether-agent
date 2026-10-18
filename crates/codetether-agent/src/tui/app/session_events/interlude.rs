//! Frontend state for shared interlude event handling.
codetether_interlude::declare_interlude_events!(
    crate::tui::app::state::App,
    crate::session::SessionEvent,
);
