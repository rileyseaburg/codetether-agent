//! Native tab management operations.

mod close;
mod list;
mod new;
mod select;

/// Close tab operation.
pub(super) use close::close;
/// List tabs operation.
pub(super) use list::list;
/// New tab operation.
pub(super) use new::new;
/// Select tab operation.
pub(super) use select::select;
