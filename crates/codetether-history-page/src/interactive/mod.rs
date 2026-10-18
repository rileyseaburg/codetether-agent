//! Interactive backward paging, generation checks and viewport anchoring.
//!
//! Construct [HistoryPageState] for a [crate::HistorySource]. Reset it when the
//! selected transcript changes, request older pages, then [drain] completed
//! pages through a [View]. Apply [apply_pending_history_anchor] after rendering.
mod apply;
mod default;
mod drain;
mod render_anchor;
mod request;
mod reset;
mod state;
mod update_view;
mod view;
mod viewport;
use crate::Page;
pub use drain::drain;
pub use render_anchor::apply_pending_history_anchor;
pub use state::HistoryPageState;
pub use update_view::UpdateView;
pub use view::View;
#[cfg(test)]
mod fixtures;
#[cfg(test)]
mod race_tests;
#[cfg(test)]
mod tests;
