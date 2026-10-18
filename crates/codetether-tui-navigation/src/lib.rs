//! Keyboard navigation priority, scrolling and cursor routing policy.
//!
//! Implement [Host] to supply current [Context] and apply [Action] operations.
//! Handlers preserve overlay priority and delegate state mutation to the host.
mod action;
mod arrows;
mod editing;
mod escape;
mod host;
mod jump;
mod paging;
mod subagent;
pub use action::Action;
pub use arrows::{handle_down, handle_up};
pub use editing::{handle_delete, handle_left, handle_right};
pub use escape::{handle_escape, toggle_help};
pub use host::{Context, Host};
pub use jump::{handle_end, handle_home};
pub use paging::{handle_page_down, handle_page_up};
