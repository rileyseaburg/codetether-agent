//! Native editor state and user-authorized journal bindings.
use crate::tui::app::state::App;
mod journal;
mod read;
mod session;
mod write;
pub(super) use session::Runtime;
pub(crate) struct Editor<'a> {
    pub app: &'a mut App,
}
