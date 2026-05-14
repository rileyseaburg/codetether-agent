//! Navigation action routing.

mod lifecycle;
mod page;

/// Navigation lifecycle actions.
pub(in crate::tool::browserctl) use lifecycle::{health, snapshot, start, stop};
/// Page navigation actions.
pub(in crate::tool::browserctl) use page::{back, goto, reload};
