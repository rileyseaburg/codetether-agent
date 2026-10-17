//! Navigation action routing.

mod lifecycle;
mod page;

/// Navigation lifecycle actions.
pub(crate) use lifecycle::{health, snapshot, start, stop};
/// Page navigation actions.
pub(crate) use page::{back, goto, reload};
