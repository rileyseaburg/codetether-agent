//! Native session payload specialization of shared autochat state.
pub type AutochatState =
    codetether_autochat_core::interactive::state::AutochatState<crate::session::SessionEvent>;
