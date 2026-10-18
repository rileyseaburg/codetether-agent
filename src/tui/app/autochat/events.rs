//! Native session payload specialization of shared autochat events.
pub type AutochatUiEvent =
    codetether_autochat_core::interactive::events::AutochatUiEvent<crate::session::SessionEvent>;
